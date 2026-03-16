use quote::{format_ident, quote};

use crate::{Args, Error};

use super::{Base, Field};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    #[serde(rename = "$key$")]
    pub name: String,

    #[serde(default)]
    pub extends: Base,

    #[serde(default)]
    pub fields: Vec<Field>,

    #[serde(default)]
    pub doc: Option<String>,
}

impl Product {
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, Error> {
        let ident = format_ident!("{}", &self.name);
        let fields: Vec<_> = self.fields.iter().map(|f| f.run(args)).try_collect()?;
        let base_fields: Vec<_> = self
            .extends
            .fields()
            .iter()
            .map(|f| f.run(args))
            .try_collect()?;

        Ok(quote! {
            use crate::*;

            pub struct #ident {
                #(#base_fields,)*
                #(#fields,)*
            }
        })
    }
}
