use druid::Color;
use proc_macro::TokenStream;
use syn::LitStr;

#[macro_use]
extern crate quote;

#[proc_macro_derive(StateEnum, attributes(color))]
pub fn state_enum(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let variants = match input.data {
        syn::Data::Enum(enum_item) => enum_item.variants,
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let len = variants.len();

    let titles: Vec<_> = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let var = variant.ident.to_string();
            quote! {
                (#var, #i),
            }
        })
        .collect();
    let colors: Vec<_> = variants
        .iter()
        .map(|variant| {
            let color = variant
                .attrs
                .iter()
                .find(|&attr| attr.path().is_ident("color"))
                .expect("color attribute is required");
            let color: LitStr = color.parse_args().expect("invalid color");
            let color = Color::from_hex_str(&color.value());
            let (r, g, b, a) = color.unwrap().as_rgba8();
            quote! {
                Color::rgba8(#r, #g, #b, #a),
            }
        })
        .collect();

    let expanded = quote! {
        impl EnumSize for #name {
            const N_STATES: usize = #len;
        }
        impl StateEnum for #name {
            const TITLES: [(&'static str, usize); Self::N_STATES] = [#(#titles)*];
            const COLOURS: [Color; Self::N_STATES] = [#(#colors)*];


            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn from_usize(s: usize) -> Self {
                debug_assert!(s < Self::N_STATES);
                unsafe { std::mem::transmute(s) }
            }
        }
    };
    expanded.into()
}
