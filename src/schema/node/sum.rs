use quote::{format_ident, quote};

use crate::{Args, Error};

use super::{Field, Variant};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SumVariant {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub fields: Vec<Field>,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sum {
    #[serde(rename = "$key$")]
    pub name: String,

    #[serde(default)]
    pub variants: Vec<Variant>,

    #[serde(default)]
    pub doc: Option<String>,
}

impl Sum {
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, Error> {
        let ident = format_ident!("{}", &self.name);
        let variants: Vec<_> = self.variants.iter().map(|v| v.run(args)).try_collect()?;

        Ok(quote! {
            pub enum #ident {
                #(#variants,)*
            }
        })
    }
}
