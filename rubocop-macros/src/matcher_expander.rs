use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::spanned::Spanned;
use syn::*;

pub fn expand(
    fn_ident: Ident,
    param_ident: Ident,
    param_type: Type,
    return_type: Type,
    pat: Pat,
) -> TokenStream {
    let param_ident = param_ident.to_token_stream();

    let mut expander = MatcherExpander {
        variables: Vec::new(),
    };

    let body = expander.expand_match(&param_ident, &pat, None);

    let MatcherExpander { variables } = expander;

    quote! {
        fn #fn_ident(#param_ident: #param_type) -> #return_type {
            #(let #variables;);*
            #body
            Some(( #(#variables),* ))
        }
    }
}

struct MatcherExpander {
    variables: Vec<Ident>,
}

impl MatcherExpander {
    fn guard_label(depth: usize) -> Lifetime {
        Lifetime {
            apostrophe: Span::call_site(),
            ident: format_ident!("__rubocop_guard_{depth}"),
        }
    }

    fn expand_match(&mut self, expr: &TokenStream, pat: &Pat, guard: Option<usize>) -> TokenStream {
        let mut arms = self.expand_match_arms(pat, guard);
        arms.push(match guard {
            Some(depth) => {
                let label = Self::guard_label(depth);
                quote! { _ => break #label false }
            }
            None => {
                quote! { _ => return None }
            }
        });
        quote! { match #expr { #(#arms)* } }
    }

    fn expand_match_arms(&mut self, pat0: &Pat, guard: Option<usize>) -> Vec<TokenStream> {
        match pat0 {
            Pat::Or(PatOr { cases, .. }) => {
                let depth = guard.map(|d| d + 1).unwrap_or(0);
                let label = Self::guard_label(depth);
                let arms = cases.iter().enumerate().map(|(i, p)| {
                    let var = format_ident!("__rubocop_or_case_{i}").to_token_stream();
                    let guard_body = self.expand_match(&var, p, Some(depth));
                    let body = self.expand_match(&var, p, guard);
                    quote! { #var if #label: { #guard_body true } => #body }
                });
                arms.collect()
            }

            Pat::Paren(PatParen { pat, .. }) => self.expand_match_arms(pat, guard),

            p => vec![self.expand_single_match_arm(p, guard)],
        }
    }

    fn expand_single_match_arm(&mut self, pat0: &Pat, guard: Option<usize>) -> TokenStream {
        match pat0 {
            Pat::Struct(PatStruct {
                path, fields, rest, ..
            }) => {
                let mut variables = Vec::with_capacity(fields.len());
                let mut statements = Vec::with_capacity(fields.len());
                for (i, FieldPat { member, pat, .. }) in fields.iter().enumerate() {
                    let Member::Named(member) = member else { unreachable!() };
                    let var = format_ident!("__rubocop_struct_field_{i}");
                    variables.push(quote! { #member: #var });
                    statements.push({
                        let expr = if contains_pat(pat, &matches_as_ref) {
                            // Box<Node>
                            quote! { #var.as_ref() }
                        } else if contains_pat(pat, &|p| matches_optional(p, &matches_as_ref)) {
                            // Option<Box<Node>>
                            quote! { #var.as_deref() }
                        } else if contains_pat(pat, &|p| matches!(p, Pat::Slice(_))) {
                            // Vec<Node>
                            quote! { #var.as_slice() }
                        } else if contains_pat(pat, &matches_byte_str) {
                            // Bytes
                            quote! { #var.raw.as_slice() }
                        } else {
                            quote! { #var }
                        };
                        self.expand_match(&expr, pat, guard)
                    });
                }
                variables.push(match rest {
                    Some(r) => quote! { #r },
                    None => quote! { .. },
                });
                quote! { #path { #(#variables),* } => { #(#statements)* } }
            }

            Pat::TupleStruct(PatTupleStruct { path, elems, .. }) => {
                let mut variables = Vec::with_capacity(elems.len());
                let mut statements = Vec::with_capacity(elems.len());
                for (i, elem) in elems.iter().enumerate() {
                    if contains_pat(elem, &|p| matches!(p, Pat::Rest(_))) {
                        todo!("TupleStruct");
                    }
                    let var = format_ident!("__rubocop_tuple_field_{i}").to_token_stream();
                    variables.push(quote! { #var });
                    statements.push(self.expand_match(&var, elem, guard));
                }
                quote! { #path( #(#variables),* ) => { #(#statements)* } }
            }

            Pat::Slice(PatSlice { elems, .. }) => {
                let mut variables = Vec::with_capacity(elems.len());
                let mut statements = Vec::with_capacity(elems.len());
                for (i, elem) in elems.iter().enumerate() {
                    let var = format_ident!("__rubocop_slice_elem_{i}").to_token_stream();
                    if contains_pat(elem, &|p| matches!(p, Pat::Rest(_))) {
                        variables.push(quote! { #var @ .. });
                    } else {
                        variables.push(quote! { #var });
                    }
                    statements.push(self.expand_match(&var, elem, guard));
                }
                quote! { [ #(#variables),* ] => { #(#statements)* } }
            }

            Pat::Ident(PatIdent { ident, subpat, .. }) => {
                if ident == "None" {
                    assert!(subpat.is_none());
                    quote! { #ident => {} }
                } else {
                    if !self.variables.contains(ident) {
                        self.variables.push(ident.clone());
                    }
                    let var = format_ident!("__rubocop_tmp_{ident}").to_token_stream();
                    let mut statements = Vec::with_capacity(2);
                    if let Some((_, p)) = subpat {
                        statements.push(self.expand_match(&var, p, guard));
                    }
                    if guard.is_none() {
                        statements.push(quote! { #ident = #var; });
                    }
                    quote! { #var => { #(#statements)* } }
                }
            }

            Pat::Rest(rest) => quote! { [ #rest ] => {} },

            Pat::Lit(lit) => quote! { #lit => {} },
            Pat::Path(path) => quote! { #path => {} },
            Pat::Range(range) => quote! { #range => {} },
            Pat::Wild(wild) => quote! { #wild => {} },

            Pat::Const(_) => todo!("Const"),
            Pat::Macro(_) => todo!("Macro"),
            Pat::Or(_) => todo!("Or"),
            Pat::Paren(_) => todo!("Paren"),
            Pat::Reference(_) => todo!("Reference"),
            Pat::Tuple(_) => todo!("Tuple"),
            Pat::Type(_) => todo!("Type"),
            Pat::Verbatim(_) => todo!("Verbatim"),

            p => match p.span().source_text() {
                Some(source) => todo!("{source}"),
                _ => todo!(),
            },
        }
    }
}

fn matches_byte_str(pat: &Pat) -> bool {
    match pat {
        Pat::Lit(PatLit {
            lit: Lit::ByteStr(_),
            ..
        }) => true,

        _ => false,
    }
}

fn matches_as_ref(pat: &Pat) -> bool {
    match pat {
        Pat::TupleStruct(PatTupleStruct {
            path: Path { segments, .. },
            ..
        }) => {
            let segments = segments.iter().collect::<Vec<_>>();
            let [a, _b] = segments[..] else { return false };

            a.ident == "Node"
        }

        Pat::Lit(ExprLit {
            lit: Lit::Str(_), ..
        }) => true,

        _ => false,
    }
}

fn matches_optional<F: Fn(&Pat) -> bool>(root: &Pat, f: &F) -> bool {
    match root {
        Pat::TupleStruct(PatTupleStruct {
            path: Path { segments, .. },
            elems,
            ..
        }) => {
            let segments = segments.iter().collect::<Vec<_>>();
            let [s] = segments[..] else { return false };

            let elems = elems.iter().collect::<Vec<_>>();
            let [p] = elems[..] else { return false };

            s.ident == "Some" && contains_pat(p, f)
        }

        _ => false,
    }
}

fn contains_pat<F: Fn(&Pat) -> bool>(root: &Pat, f: &F) -> bool {
    match root {
        Pat::Ident(PatIdent {
            subpat: Some((_, p)),
            ..
        }) => contains_pat(p, f),

        Pat::Or(PatOr { cases, .. }) => cases.iter().any(|p| contains_pat(p, f)),

        Pat::Paren(PatParen { pat, .. }) => contains_pat(pat, f),

        p => f(p),
    }
}
