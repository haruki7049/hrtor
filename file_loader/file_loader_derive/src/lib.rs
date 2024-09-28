use proc_macro::TokenStream;
use file_loader::FileInfo;

#[proc_macro_derive(CommandLineArgsParser)]
pub fn read_fileinfo(input: TokenStream) -> TokenStream {
    let _ = input;

    unimplemented!()
}
