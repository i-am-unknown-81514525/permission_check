use permission_parser::{parser, tokenizer::{Field, ListSpecifier, Specifier}, Permissions, Expr};
use proc_macro2::{Span};
use proc_macro::{TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input};

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
enum Token {
    Field(Field),
    ListSpecifier(ListSpecifier),
    Specifier(Specifier)
}

impl From<Field> for Token {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

fn enum_to_token(parsed_enum: Token) -> impl ToTokens {
    match parsed_enum {
        Token::Field(field) => {
            match field {
                Field::Name { name } => {
                    let name = syn::LitStr::new(&name, Span::call_site());
                    quote! { ::permission_parser::tokenizer::Field::Name { name : (#name).to_string()}}
                },
                Field::ID { id } => quote! { ::permission_parser::tokenizer::Field::ID {id: #id} },
                Field::Specifier { specifier } => {
                    let inner = enum_to_token(Token::Specifier(specifier));
                    quote! {::permission_parser::tokenizer::Field::Specifier {specifier: #inner } }
                },
                Field::Glob => quote! { ::permission_parser::tokenizer::Field::Glob },
                Field::DoubleGlob => quote! { ::permission_parser::tokenizer::Field::DoubleGlob },
                Field::TripleGlob => quote! { ::permission_parser::tokenizer::Field::TripleGlob },
                Field::VarKind(span, ident) => {
                    quote_spanned! {
                        span => 
                        {
                            fn converter<T: ::std::string::ToString>(v: &T) -> ::permission_parser::tokenizer::Field {
                                ::permission_parser::tokenizer::Field::Name {name: v.to_string()}
                            }

                            converter(&#ident)
                        }
                    }
                }
            }
        },
        Token::Specifier(specifier) => {
            match specifier {
                Specifier::Assign => quote! { ::permission_parser::tokenizer::Specifier::Assign },
                Specifier::Read => quote! { ::permission_parser::tokenizer::Specifier::Read },
                Specifier::Write => quote! { ::permission_parser::tokenizer::Specifier::Write },
                Specifier::Enact => quote! { ::permission_parser::tokenizer::Specifier::Enact },
                Specifier::ListSpecifier { specifier }  => {
                    let inner = enum_to_token(Token::ListSpecifier(specifier));
                    quote! {::permission_parser::tokenizer::Specifier::ListSpecifier {specifier: #inner} }
                },
            }
        },
        Token::ListSpecifier(specifier) => {
            match specifier {
                ListSpecifier::Add => quote! { ::permission_parser::tokenizer::ListSpecifier::Add },
                ListSpecifier::Remove => quote! { ::permission_parser::tokenizer::ListSpecifier::Remove },
                ListSpecifier::ReadOne => quote! { ::permission_parser::tokenizer::ListSpecifier::ReadOne },
                ListSpecifier::ListAll => quote! { ::permission_parser::tokenizer::ListSpecifier::ListAll },
            }
        }
    }
}

fn to_internal_token(permissions: &Permissions) -> Vec<Token> {
    parser::token_converter(permissions.clone()).unwrap().iter().map(|x| (*x).clone().into()
    ).collect()
}

fn perm_reconstructor(input: Vec<Token>) -> impl ToTokens {
    let code_token: Vec<_> = input.iter().map(|token| enum_to_token((*token).clone()) ).collect();
    quote! {
        ::permission_parser::parser::PermissionItem {
            perm: vec![
                #(#code_token),*
            ]
        }
    }
}

#[proc_macro]
pub fn perm_parser(input: TokenStream) -> TokenStream {
    let input: Vec<Token> = to_internal_token(&parse_macro_input!(input as parser::Permissions));
    let v = perm_reconstructor(input);
    quote! {#v}.into()
}

fn expr_to_token(expr: Expr) -> impl ToTokens {
    match expr {
        Expr::Permission(permissions) => {

            let tokens = perm_reconstructor(to_internal_token(&permissions));
            quote! {
                ::permission_check::check(&#tokens, var)
            }
        },
        Expr::And(left, right) => {
            let left = expr_to_token(*left);
            let right = expr_to_token(*right);
            quote! {
                (#left && #right)
            }
        },
        Expr::Or(left, right) => {
            let left = expr_to_token(*left);
            let right = expr_to_token(*right);
            quote! {
                (#left || #right)
            }
        },
        Expr::Xor(left, right) => {
            let left = expr_to_token(*left);
            let right = expr_to_token(*right);
            quote! {
                (#left ^ #right)
            }
        },
        Expr::Not(item) => {
            let item = expr_to_token(*item);
            quote! {
                (!(#item))
            }
        },
        Expr::Bracketed(item) => {
            let item = expr_to_token(*item);
            quote! {
                (#item)
            }
        },
    }
}

#[proc_macro]
pub fn perm_expr(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as Expr);
    let token_content = expr_to_token(output);
    let expanded = quote! {
        ::permission_check::ComplexCheck::new(::std::boxed::Box::new(move |var| #token_content))
    };
    expanded.into()
}