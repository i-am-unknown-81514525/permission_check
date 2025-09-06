use permission_parser::{parser, tokenizer::{Field, ListSpecifier, Specifier}, Permission, Permissions};
use proc_macro2::{Span};
use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::{Parse, ParseStream}, parse_macro_input, token::Paren};

#[derive(Clone)]
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
                Field::TripleGlob => quote! { ::permission_parser::tokenizer::Field::TripleGlob }
            }
        },
        Token::Specifier(specifier) => {
            match specifier {
                Specifier::Assign => quote! { ::permission_parser::tokenizer::Specifier::Assign },
                Specifier::Read => quote! { ::permission_parser::tokenizer::Specifier::Read },
                Specifier::Write => quote! { ::permission_parser::tokenizer::Specifier::Write },
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

enum Expr {
    Permission(Permissions),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Bracketed(Box<Expr>)
}


impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            return Ok(Self::Bracketed(content.parse()?));
        }
        if input.peek(syn::Token![!]) {
            input.parse::<syn::Token![!]>()?;
            let expr: Expr = input.parse()?;
            return Ok(Self::Not(Box::new(expr)));
        }
        if input.fork().parse::<Permissions>().is_ok() {
            let parsed: Permissions =  input.parse()?;
            let expr: Box<Self> = Box::new(
                Self::Permission(
                    parsed
                )
            );
            if input.peek(syn::Token![&]) || input.peek(syn::Token![&&]) {
                if input.peek(syn::Token![&]) {
                    input.parse::<syn::Token![&]>()?;
                } else {
                    input.parse::<syn::Token![&&]>()?;
                }
                let right: Expr = input.parse()?;
                return Ok(Self::And(expr, Box::new(right)));
            } else if input.peek(syn::Token![|]) || input.peek(syn::Token![||]) {
                if input.peek(syn::Token![|]) {
                    input.parse::<syn::Token![|]>()?;
                } else {
                    input.parse::<syn::Token![||]>()?;
                }
                let right: Expr = input.parse()?;
                return Ok(Self::Or(expr, Box::new(right)));
            } else if input.peek(syn::Token![^])  {
                input.parse::<syn::Token![^]>()?;
                let right: Expr = input.parse()?;
                return Ok(Self::Xor(expr, Box::new(right)));
            }
            return Ok(*expr);
        }
        if input.fork().parse::<Permission>().is_ok() {
            input.parse::<Permissions>()?;
        } else {
            return Err(syn::Error::new(input.span(), "Invalid token"))
        }
        unreachable!()
    }
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
        (|var| #token_content)
    };
    expanded.into()
}