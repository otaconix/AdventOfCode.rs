use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput};

#[proc_macro_derive(EnumVariants)]
pub fn derive_enum_variants(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    if let Data::Enum(enum_data) = ast.data {
        let ident = ast.ident;
        let variant_count = enum_data.variants.len();
        let variants = enum_data.variants.iter();

        let implementation = quote! {
            #[automatically_derived]
            impl EnumVariants<#variant_count> for #ident {
                fn variants() -> [Self; #variant_count] {
                    [
                        #(#ident::#variants),*,
                    ]
                }
            }
        };

        implementation.into()
    } else {
        todo!()
    }
}
