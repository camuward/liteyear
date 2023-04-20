use proc_macro::TokenStream;

#[cfg(not(bench))]
#[proc_macro_attribute]
pub fn bench(_attr: TokenStream, item_fn: TokenStream) -> TokenStream {
    item_fn
}

#[cfg(bench)]
mod bench;

#[cfg(bench)]
#[proc_macro_attribute]
pub fn bench(attr: TokenStream, item_fn: TokenStream) -> TokenStream {
    bench::bench(attr.into(), item_fn.into()).into()
}
