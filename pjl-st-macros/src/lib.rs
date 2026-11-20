use proc_macro::*;

#[proc_macro_attribute]
pub fn st_primitive(attr:TokenStream, item:TokenStream) -> TokenStream {
    eprintln!("st_primitive attribute called with attr: {:?}\n item: {:#?}", attr, item);
    TokenStream::new()
}
