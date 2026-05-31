use super::Type;
use crate::ast::Path;
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::As;
use crate::token::punct::{Gt, Lt};
use crate::{Span, TokenStream};

#[doc = "The `<T as Trait>` qualifier of a qualified path."]
#[derive(Debug, Clone)]
pub struct QSelf {
    pub span: Span,
    pub ty: Box<Type>,
    /// Number of leading path segments that belong inside the `<... as Trait>`.
    pub position: usize,
}

impl QSelf {
    /// Parse `< Type ( as Path )? >`, returning the qself plus the trait path
    /// segments (if any) that the enclosing `TypePath` must prepend to its path.
    pub(super) fn parse_with_trait(
        stream: &mut ParseStream,
    ) -> Result<(Self, Option<Path>), ParseError> {
        let _ = stream.parse::<Lt>()?;
        let ty = Box::new(stream.parse::<Type>()?);

        let trait_path = if stream.peek::<As>().is_some() {
            let _ = stream.parse::<As>()?;
            Some(stream.parse::<Path>()?)
        } else {
            None
        };

        let _ = stream.parse::<Gt>()?;

        let position = trait_path.as_ref().map(|p| p.segments.len()).unwrap_or(0);

        Ok((
            Self {
                span: Span::default(),
                ty,
                position,
            },
            trait_path,
        ))
    }
}

impl ToTokens for QSelf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Emits just `< ty >`; the `as Trait` portion is rendered by `TypePath`
        // (it owns the trait segments and the closing `>`/`::` placement).
        Lt::default().to_tokens(tokens);
        self.ty.to_tokens(tokens);
        Gt::default().to_tokens(tokens);
    }
}
