use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("async_trait.wasm");

#[proc_macro_attribute]
pub fn async_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    MACRO.proc_macro_attribute("async_trait", args, input)
}
