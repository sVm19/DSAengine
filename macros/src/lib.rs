extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs, NestedMeta, Lit, Meta};

/// #[mcp_tool(name = "tool_name", description = "...")]
///
/// Decorates an Axum handler and emits two companion constants
/// (`TOOL_NAME` and `TOOL_DESCRIPTION`) that downstream code can
/// read for discovery/manifest generation.
#[proc_macro_attribute]
pub fn mcp_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let func = parse_macro_input!(item as ItemFn);

    let mut tool_name = String::new();
    let mut tool_description = String::new();

    for arg in &args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("name") {
                if let Lit::Str(s) = &nv.lit {
                    tool_name = s.value();
                }
            } else if nv.path.is_ident("description") {
                if let Lit::Str(s) = &nv.lit {
                    tool_description = s.value();
                }
            }
        }
    }

    let expanded = quote! {
        /// The MCP tool name for this skill, extracted from the `#[mcp_tool]` attribute.
        pub const TOOL_NAME: &str = #tool_name;
        /// The MCP tool description for this skill.
        pub const TOOL_DESCRIPTION: &str = #tool_description;

        #func
    };

    expanded.into()
}
