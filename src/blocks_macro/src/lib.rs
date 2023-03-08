use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{self, braced, Ident};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, LitBool, Result, Token,
};

struct MyBlock {
    name: Ident,
    is_transparent: LitBool,
    color: Expr,
}

impl Parse for MyBlock {
    fn parse(input: ParseStream<'_>) -> Result<MyBlock> {
        mod kw {
            ::syn::custom_keyword!(is_transparent);
            ::syn::custom_keyword!(color);
        }
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;

        let content;
        braced!(content in input);
        let _: kw::is_transparent = content.parse()?;
        let _: Token![:] = content.parse()?;
        let is_transparent: LitBool = content.parse()?;
        let _: Token![,] = content.parse()?;
        let _: kw::color = content.parse()?;
        let _: Token![:] = content.parse()?;
        let color: Expr = content.parse()?;
        if content.peek(Token![,]) {
            let _: Token![,] = content.parse()?;
        }

        Ok(Self {
            name,
            is_transparent,
            color,
        })
    }
}

#[proc_macro]
pub fn create_blocks(input: TokenStream) -> TokenStream {
    let blocks: Punctuated<MyBlock, Token![,]> =
        parse_macro_input!(input with Punctuated::parse_terminated);

    let blocks_array = generate_blocks_array(&blocks);
    let block_id_function = generate_get_block_id_function(&blocks);
    let blocks_ids_mod = genereate_blocks_ids_module(&blocks);
    let expanded = quote! {
        pub struct BlockInfos {
            pub id: u16,
            pub name: &'static str,
            pub is_transparent: bool,
            pub color: Option<Color>,
        }
        #blocks_ids_mod
        #blocks_array
        #block_id_function
    };

    expanded.into()
}

fn generate_blocks_array(blocks: &Punctuated<MyBlock, Token![,]>) -> proc_macro2::TokenStream {
    let mut array_content = quote! {};
    for (i, block) in blocks.iter().enumerate() {
        let block_id = i as u16;
        let name = block.name.to_string().to_lowercase();
        let is_transparent = block.is_transparent.clone();
        let color = block.color.clone();
        let new_block = quote! {
                BlockInfos {
                    id: #block_id,
                    name: #name,
                    is_transparent: #is_transparent,
                    color: #color,
                },
        };
        array_content = quote! {#array_content #new_block};
    }

    let nb_blocks = blocks.len();
    let array = quote! {
        const BLOCKS: [BlockInfos; #nb_blocks] = [ #array_content ];
    };

    array
}

fn generate_get_block_id_function(
    blocks: &Punctuated<MyBlock, Token![,]>,
) -> proc_macro2::TokenStream {
    let mut match_content = quote! {};
    for (i, block) in blocks.iter().enumerate() {
        let block_id = i as u16;
        let block_name = block.name.to_string().to_lowercase();
        match_content = quote! {
            #match_content
            #block_name => #block_id,
        };
    }

    let function = quote! {
        pub fn get_block_id(block_name: &str) -> u16 {
            match block_name {
                #match_content
                _ => panic!("The block name : {} does_not exist", block_name),
            }
        }
    };

    function
}

fn genereate_blocks_ids_module(
    blocks: &Punctuated<MyBlock, Token![,]>,
) -> proc_macro2::TokenStream {
    let mut mod_content = quote! {};
    for (i, block) in blocks.iter().enumerate() {
        let block_id = i as u16;
        let block_name = block.name.clone();
        mod_content = quote! {
            #mod_content
            pub const #block_name: u16 = #block_id;
        };
    }

    let blocks_ids_mod = quote! {
        pub mod blocks_ids {
            #mod_content
        }
    };

    blocks_ids_mod
}
