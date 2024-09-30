use proc_macro::TokenStream;

#[proc_macro_derive(CommandLineArgsParser)]
pub fn read_fileinfo(input: TokenStream) -> TokenStream {
    let _ = input;

    unimplemented!()
}
