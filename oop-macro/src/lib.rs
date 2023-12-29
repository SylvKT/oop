use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, parse::Parse, Type};

struct ExtendArgs {
	target: Box<Type>,
	field_name: TokenTree,
}

impl Parse for ExtendArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let target = input.parse()?;
		input.parse::<syn::Token![,]>()?;
		let field_name = input.parse()?;
		Ok(
			Self {
				target,
				field_name,
			}
		)
    }
}

#[proc_macro_attribute]
pub fn extend(attr: TokenStream, mut item: TokenStream) -> TokenStream {
	let item2 = item.clone();
	let input = parse_macro_input!(item2 as ItemStruct);
	let args = parse_macro_input!(attr as ExtendArgs);
	
	let struct_name = input.ident;
	let target = args.target;
	let field_name = args.field_name;
	item.extend::<TokenStream>(
		quote! {
			use core::ops::{Deref, DerefMut};
			
			impl Deref for #struct_name {
				type Target = #target;
				
				fn deref(&self) -> &Self::Target {
					&self.#field_name
				}
			}
			
			impl DerefMut for #struct_name {
				fn deref_mut(&mut self) -> &mut Self::Target {
					&mut self.#field_name
				}
			}
			
			impl #struct_name {
				pub fn _super(&self) -> &#target {
					self.deref()
				}
				
				pub fn _super_mut(&mut self) -> &mut #target {
					self.deref_mut()
				}
			}
		}.into()
	);
	
	item
}
