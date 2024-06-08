use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::{FnArg, ItemTrait, Pat, TraitItem};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn delegate_app(_: TokenStream, input: TokenStream) -> TokenStream {
    parse(input).unwrap_or_else(|e| e.into_compile_error().into())
}

fn parse(input: TokenStream) -> syn::Result<TokenStream> {
    let mut trait_item = syn::parse::<ItemTrait>(input.clone())?;
    let functions = functions(&mut trait_item);
    let ident = &trait_item.ident;
    let generics = &trait_item.generics;
    let (_, params, _) = &generics.split_for_impl();
    let input = TokenStream2::from(input);
    let expand = quote! {
        #input
        impl #generics #ident #params for bevy::prelude::App{
            #functions
        }
    };
    Ok(expand.into())
}

fn functions(trait_item: &mut ItemTrait) -> TokenStream2 {
    let functions = trait_item
        .items
        .iter_mut()
        .filter_map(|item| {
            match item {
                TraitItem::Fn(f) => Some(f),
                _ => None
            }
        })
        .map(|f| {
            let name = &f.sig.ident;
            let world = match f.sig.inputs.iter().next().unwrap() {
                FnArg::Receiver(r) => {
                    if r.mutability.is_some(){
                        quote!(world_mut())
                    }else{
                        quote!(world())
                    }
                },
                _ => {panic!()}
            };

            let inputs = &f.sig.inputs.iter().filter_map(|arg| match arg {
                FnArg::Typed(pat_type) => {
                    let ident = require_ident(&pat_type.pat).ok()?;
                    Some(quote::quote! {#ident,})
                }
                _ => None
            })
                .collect::<Vec<_>>();
            f.default.replace(syn::parse2(quote!({
                self.#world.#name(#(#inputs)*)
            })).unwrap());
            f
        });

    quote!(#(#functions)*)
}

fn require_ident(pat: &Pat) -> syn::Result<&Ident> {
    if let Pat::Ident(ident) = pat {
        Ok(&ident.ident)
    } else {
        Err(syn::Error::new(
            pat.span(),
            "Required PatIdent But Different",
        ))
    }
}
