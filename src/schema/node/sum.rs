use quote::{format_ident, quote};

use crate::{Args, Error};

use super::{TypeMeta, Variant};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SumVariant {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub fields: Vec<super::Field>,
}

impl SumVariant {
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, Error> {
        let ident = format_ident!("{}", &self.name);
        let fields: Vec<_> = self.fields.iter().map(|f| f.run(args)).try_collect()?;

        Ok(quote! {
            #ident {
                #(#fields,)*
            }
        })
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Sum {
    #[serde(rename = "$key$")]
    pub name: String,

    #[serde(default)]
    pub variants: Vec<Variant>,

    #[serde(default)]
    pub doc: Option<String>,

    #[serde(default)]
    pub submodule: Option<String>,

    #[serde(default)]
    pub meta: Option<TypeMeta>,
}

impl Sum {
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, Error> {
        let ident = format_ident!("{}", &self.name);
        let doc = self.doc.as_deref().map(|d| quote!(#[doc = #d]));
        let variants: Vec<_> = self.variants.iter().map(|v| v.run(args)).try_collect()?;

        Ok(quote! {
            #[allow(unused)]
            use super::*;

            #doc
            #[derive(Debug, Clone)]
            pub enum #ident {
                #(#variants,)*
            }
        })
    }
}
