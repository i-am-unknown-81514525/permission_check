use permission_parser::{parser, tokenizer::{Field, ListSpecifier, Specifier}};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

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
                    quote! { ::permission_parser::Field::Name { name : #name}}
                },
                Field::ID { id } => quote! { ::permission_parser::Field::ID {id: #id} },
                Field::Specifier { specifier } => {
                    let inner = enum_to_token(Token::Specifier(specifier));
                    quote! {::permission_parser::Field::Specifier(#inner)}
                },
                Field::Glob => quote! { ::permission_parser::Field::Glob },
                Field::DoubleGlob => quote! { ::permission_parser::Field::DoubleGlob },
                Field::TripleGlob => quote! { ::permission_parser::Field::TripleGlob }
            }
        },
        Token::Specifier(specifier) => {
            match specifier {
                Specifier::Assign => quote! { ::permission_parser::Specifier::Assign },
                Specifier::Read => quote! { ::permission_parser::Specifier::Read },
                Specifier::Write => quote! { ::permission_parser::Specifier::Write },
                Specifier::ListSpecifier { specifier }  => {
                    let inner = enum_to_token(Token::ListSpecifier(specifier));
                    quote! {::permission_parser::Specifier::ListSpecifier(#inner)}
                },
            }
        },
        Token::ListSpecifier(specifier) => {
            match specifier {
                ListSpecifier::Add => quote! { ::permission_parser::ListSpecifier::Add },
                ListSpecifier::Remove => quote! { ::permission_parser::ListSpecifier::Remove },
                ListSpecifier::ReadOne => quote! { ::permission_parser::ListSpecifier::ReadOne },
                ListSpecifier::ListAll => quote! { ::permission_parser::ListSpecifier::ListAll },
            }
        }
    }
}

#[proc_macro]
pub fn perm_parser(input: TokenStream) -> TokenStream {
    let input: Vec<Token> = parser::token_converter(
        parse_macro_input!(input as parser::Permissions)).unwrap().iter().map(|x| (*x).clone().into()
    ).collect();
    let code_token: Vec<_> = input.iter().map(|token| enum_to_token((*token).clone()) ).collect();
    let expanded = quote! {
        ::parser::PermissionItem {
            perm: vec![
                #(#code_token),*
            ]
        }
    };
    expanded.into()
}