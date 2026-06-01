use crate::ast::{Ident, Punctuated};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::As;
use crate::token::punct::{Comma, PathSep, Star};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A leaf name in a use tree (`foo`)."]
#[derive(Debug, Clone)]
pub struct UseName {
    pub span: Span,
    pub ident: Ident,
}

#[doc = "A renamed use leaf (`foo as bar`)."]
#[derive(Debug, Clone)]
pub struct UseRename {
    pub span: Span,
    pub ident: Ident,
    pub rename: Ident,
}

#[doc = "A use path segment (`foo::<rest>`)."]
#[derive(Debug, Clone)]
pub struct UsePath {
    pub span: Span,
    pub ident: Ident,
    pub tree: Box<UseTree>,
}

#[doc = "A braced use group (`{a, b::c}`)."]
#[derive(Debug, Clone)]
pub struct UseGroup {
    pub span: Span,
    pub items: Punctuated<UseTree, Comma>,
}

#[doc = "A `use` import tree."]
#[derive(Debug, Clone)]
pub enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob,
    Group(UseGroup),
}

impl Parse for UseTree {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // `*` glob
        if stream.peek::<Star>().is_some() {
            let _ = stream.parse::<Star>()?;
            return Ok(UseTree::Glob);
        }
        // `{ ... }` group
        if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            let items = Punctuated::parse_terminated(&mut inner)?;
            return Ok(UseTree::Group(UseGroup {
                span: Span::default(),
                items,
            }));
        }

        let ident = stream.parse::<Ident>()?;

        // `ident :: rest` → path
        if stream.peek::<PathSep>().is_some() {
            let _ = stream.parse::<PathSep>()?;
            let tree = Box::new(stream.parse::<UseTree>()?);
            return Ok(UseTree::Path(UsePath {
                span: Span::default(),
                ident,
                tree,
            }));
        }

        // `ident as rename`
        if stream.peek::<As>().is_some() {
            let _ = stream.parse::<As>()?;
            let rename = stream.parse::<Ident>()?;
            return Ok(UseTree::Rename(UseRename {
                span: Span::default(),
                ident,
                rename,
            }));
        }

        Ok(UseTree::Name(UseName {
            span: Span::default(),
            ident,
        }))
    }
}

impl ToTokens for UseTree {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            UseTree::Path(p) => {
                p.ident.to_tokens(t);
                PathSep::default().to_tokens(t);
                p.tree.to_tokens(t);
            }
            UseTree::Name(n) => n.ident.to_tokens(t),
            UseTree::Rename(r) => {
                r.ident.to_tokens(t);
                As::default().to_tokens(t);
                r.rename.to_tokens(t);
            }
            UseTree::Glob => Star::default().to_tokens(t),
            UseTree::Group(g) => {
                let mut inner = TokenStream::new();
                g.items.to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
            }
        }
    }
}
