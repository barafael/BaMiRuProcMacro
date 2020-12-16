use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn id(_ident: TokenStream, attr: TokenStream) -> TokenStream {
    attr
}

#[proc_macro_attribute]
pub fn print_attr(_ident: TokenStream, attr: TokenStream) -> TokenStream {
    dbg!(attr);
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn print_ident(ident: TokenStream, _attr: TokenStream) -> TokenStream {
    dbg!(ident);
    TokenStream::new()
}
