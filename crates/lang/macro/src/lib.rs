extern crate proc_macro;

mod contract;
mod erc165;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn import(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn metis(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// The marco to generate _supports_interface for impl erc165
/// Use like
///    #[metis::supports(interface(new, default), interface(flip, get))]
///    impl Flipper {}
/// This will generate this two interface: 
///   - Selector(new) ^ Selector(default)
///   - Selector(flip) ^ Selector(get)
#[proc_macro_attribute]
pub fn supports(attr: TokenStream, item: TokenStream) -> TokenStream {
    erc165::generate(attr.into(), item.into()).into()
}
