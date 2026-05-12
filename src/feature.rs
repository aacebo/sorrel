use quote::{format_ident, quote};

use crate::{Field, FieldMeta, Node, Product, Variant, Wrapper};

#[derive(clap::ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Feature {
    Visit,
    Fold,
}

impl Feature {
    pub fn generate(&self, node: &Node) -> proc_macro2::TokenStream {
        match self {
            Feature::Visit => generate_visit(node),
            Feature::Fold => generate_fold(node),
        }
    }

    pub fn generate_global<'a>(
        &self,
        nodes: impl Iterator<Item = &'a Node>,
    ) -> proc_macro2::TokenStream {
        let nodes: Vec<_> = nodes.collect();

        match self {
            Feature::Visit => generate_visit_global(&nodes),
            Feature::Fold => generate_fold_global(&nodes),
        }
    }
}

fn node_fields(p: &Product) -> Vec<Field> {
    p.extends
        .fields()
        .into_iter()
        .chain(p.fields.iter().cloned())
        .collect()
}

fn field_meta(f: &Field) -> Option<&FieldMeta> {
    f.meta.as_ref().filter(|m| m.node)
}

fn fold_expr(
    src: proc_macro2::TokenStream,
    wrapper: &Wrapper,
    folder: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match wrapper {
        Wrapper::None => quote! { #src.fold(#folder) },
        Wrapper::Box => quote! { Box::new((*#src).fold(#folder)) },
        Wrapper::Vec | Wrapper::Punctuated => {
            quote! { #src.into_iter().map(|x| x.fold(#folder)).collect() }
        }
        Wrapper::Option => quote! { #src.map(|x| x.fold(#folder)) },
        Wrapper::OptionBox => quote! { #src.map(|x| Box::new((*x).fold(#folder))) },
        Wrapper::OptionVec => {
            quote! { #src.map(|v| v.into_iter().map(|x| x.fold(#folder)).collect()) }
        }
    }
}

fn generate_visit(node: &Node) -> proc_macro2::TokenStream {
    let ident = format_ident!("{}", node.name());

    let body = match node {
        Node::Product(p) => {
            let method = format_ident!("visit_{}", convert_case::ccase!(snake, &p.name));
            quote! { visitor.#method(self); }
        }
        Node::Sum(s) => {
            let arms: Vec<_> = s.variants.iter().map(|v| visit_arm(&ident, v)).collect();
            quote! { match self { #(#arms,)* } }
        }
    };

    quote! {
        impl crate::ast::Visit for #ident {
            fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
                #body
            }
        }
    }
}

fn visit_arm(enum_ident: &proc_macro2::Ident, v: &Variant) -> proc_macro2::TokenStream {
    match v {
        Variant::Enum(name) => {
            let arm = format_ident!("{}", name);
            quote! { #enum_ident::#arm => {} }
        }
        Variant::Sum(sv) => {
            let variant = format_ident!("{}", &sv.name);

            if sv.fields.is_empty() {
                return quote! { #enum_ident::#variant {} => {} };
            }

            let names: Vec<_> = sv
                .fields
                .iter()
                .map(|f| format_ident!("{}", &f.name))
                .collect();
            let stmts: Vec<_> = sv.fields.iter().map(visit_stmt).collect();

            quote! { #enum_ident::#variant { #(#names,)* } => { #(#stmts)* } }
        }
    }
}

fn visit_stmt(f: &Field) -> proc_macro2::TokenStream {
    let field = format_ident!("{}", &f.name);

    let Some(meta) = field_meta(f) else {
        return quote! { let _ = &#field; };
    };

    match &meta.wrapper {
        Wrapper::None => quote! { #field.visit(visitor); },
        Wrapper::Box => quote! { (*#field).visit(visitor); },
        Wrapper::Vec | Wrapper::Punctuated => {
            quote! { for x in #field.iter() { x.visit(visitor); } }
        }
        Wrapper::Option => quote! { if let Some(x) = #field { x.visit(visitor); } },
        Wrapper::OptionBox => {
            quote! { if let Some(x) = #field { (**x).visit(visitor); } }
        }
        Wrapper::OptionVec => {
            quote! { if let Some(v) = #field { for x in v { x.visit(visitor); } } }
        }
    }
}

fn generate_visit_global(nodes: &[&Node]) -> proc_macro2::TokenStream {
    let methods: Vec<_> = nodes.iter().map(|n| visitor_method(n)).collect();

    quote! {
        pub trait Visit {
            fn visit(&self, visitor: &mut impl Visitor);
        }

        pub trait Visitor {
            #(#methods)*
        }
    }
}

fn visitor_method(node: &Node) -> proc_macro2::TokenStream {
    let ident = format_ident!("{}", node.name());
    let method = format_ident!("visit_{}", convert_case::ccase!(snake, node.name()));

    match node {
        Node::Product(_) => {
            quote! { fn #method(&mut self, node: &#ident) {} }
        }
        Node::Sum(_) => {
            quote! {
                fn #method(&mut self, node: &#ident) where Self: Sized {
                    crate::ast::Visit::visit(node, self);
                }
            }
        }
    }
}

fn generate_fold(node: &Node) -> proc_macro2::TokenStream {
    let ident = format_ident!("{}", node.name());

    let body = match node {
        Node::Product(p) => {
            let method = format_ident!("fold_{}", convert_case::ccase!(snake, &p.name));
            quote! { folder.#method(self) }
        }
        Node::Sum(s) => {
            let arms: Vec<_> = s.variants.iter().map(|v| fold_arm(&ident, v)).collect();
            quote! { match self { #(#arms,)* } }
        }
    };

    quote! {
        impl crate::ast::Fold for #ident {
            fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
                #body
            }
        }
    }
}

fn fold_arm(enum_ident: &proc_macro2::Ident, v: &Variant) -> proc_macro2::TokenStream {
    match v {
        Variant::Enum(name) => {
            let arm = format_ident!("{}", name);
            quote! { #enum_ident::#arm => #enum_ident::#arm }
        }
        Variant::Sum(sv) => {
            let variant = format_ident!("{}", &sv.name);

            if sv.fields.is_empty() {
                return quote! { #enum_ident::#variant {} => #enum_ident::#variant {} };
            }

            let names: Vec<_> = sv
                .fields
                .iter()
                .map(|f| format_ident!("{}", &f.name))
                .collect();
            let inits: Vec<_> = sv.fields.iter().map(fold_init_variant).collect();

            quote! {
                #enum_ident::#variant { #(#names,)* } => #enum_ident::#variant { #(#inits,)* }
            }
        }
    }
}

fn fold_init_variant(f: &Field) -> proc_macro2::TokenStream {
    let field = format_ident!("{}", &f.name);

    let Some(meta) = field_meta(f) else {
        return quote! { #field };
    };

    fold_expr(quote! { #field }, &meta.wrapper, quote! { folder })
}

fn generate_fold_global(nodes: &[&Node]) -> proc_macro2::TokenStream {
    let methods: Vec<_> = nodes
        .iter()
        .filter_map(|n| match n {
            Node::Product(p) => Some(folder_method(p)),
            Node::Sum(_) => None,
        })
        .collect();

    quote! {
        pub trait Fold: Sized {
            fn fold(self, folder: &mut impl Folder) -> Self;
        }

        pub trait Folder {
            #(#methods)*
        }
    }
}

fn folder_method(p: &Product) -> proc_macro2::TokenStream {
    let ident = format_ident!("{}", &p.name);
    let method = format_ident!("fold_{}", convert_case::ccase!(snake, &p.name));
    let inits: Vec<_> = node_fields(p)
        .iter()
        .map(fold_init_product)
        .collect();

    quote! {
        fn #method(&mut self, node: #ident) -> #ident where Self: Sized {
            #ident { #(#inits,)* }
        }
    }
}

fn fold_init_product(f: &Field) -> proc_macro2::TokenStream {
    let field = format_ident!("{}", &f.name);

    let Some(meta) = field_meta(f) else {
        return quote! { #field: node.#field };
    };

    let expr = fold_expr(quote! { node.#field }, &meta.wrapper, quote! { self });

    quote! { #field: #expr }
}
