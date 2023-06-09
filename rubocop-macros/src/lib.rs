mod matcher_expander;

use proc_macro::TokenStream;
use quote::*;
use syn::parse::*;
use syn::*;

#[proc_macro]
pub fn node_matcher(input: TokenStream) -> TokenStream {
    let MatcherArgs {
        fn_ident,
        param_ident,
        param_type,
        return_type,
        pat,
    } = parse_macro_input!(input as MatcherArgs);

    TokenStream::from(matcher_expander::expand(
        fn_ident,
        param_ident,
        param_type,
        return_type,
        pat,
    ))
}

struct MatcherArgs {
    fn_ident: Ident,
    param_ident: Ident,
    param_type: Type,
    return_type: Type,
    pat: Pat,
}

impl Parse for MatcherArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![fn]>()?;
        let fn_ident = input.parse::<Ident>()?;

        let content;
        parenthesized!(content in input);
        let param_ident = content.parse::<Ident>()?;
        content.parse::<Token![:]>()?;
        let param_type = content.parse::<Type>()?;

        input.parse::<Token![->]>()?;
        let return_type = input.parse::<Type>()?;

        input.parse::<Token![,]>()?;
        let pat = input.call(Pat::parse_multi_with_leading_vert)?;

        Ok(Self {
            fn_ident,
            param_ident,
            param_type,
            return_type,
            pat,
        })
    }
}

#[proc_macro_derive(AutoCorrector)]
pub fn auto_corrector_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);
    TokenStream::from(quote! { impl AutoCorrector for #ident {} })
}
