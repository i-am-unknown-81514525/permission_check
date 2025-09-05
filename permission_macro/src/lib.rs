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

#[proc_macro]
pub fn perm_parser(input: TokenStream) -> TokenStream {
    let input: Vec<Token> = parser::token_converter(
        parse_macro_input!(input as parser::Permissions)).unwrap().iter().map(|x| (*x).clone().into()
    ).collect();
    let code_token: Vec<_> = input.iter().map(|token| enum_to_token((*token).clone()) ).collect();
    let expanded = quote! {
        ::permission_parser::parser::PermissionItem {
            perm: vec![
                #(#code_token),*
            ]
        }
    };
    expanded.into()
}