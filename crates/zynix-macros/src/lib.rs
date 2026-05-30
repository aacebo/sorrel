extern crate proc_macro;

#[proc_macro]
pub fn parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ts: zynix::TokenStream = input.into();
    ts.into()
}
