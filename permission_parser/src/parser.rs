
use crate::{token, tokenizer};
use regex::Regex;
use std::{sync::LazyLock};
use syn::{
    LitInt, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// Part for Field exclude triple glob
// ([a-zA-Z0-9]+|\*|\*\*)
// (\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))*|(([a-zA-Z0-9]+|\*|\*\*)\.)*(\*\*\*|([a-zA-Z0-9]+|\*|\*\*))|(([a-zA-Z0-9]+|\*|\*\*)\.)+\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))+)

fn match_strings(permission: &String) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))*|(([a-zA-Z0-9]+|\*|\*\*)\.)*(\*\*\*|([a-zA-Z0-9]+|\*|\*\*))|(([a-zA-Z0-9]+|\*|\*\*)\.)+\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))+)$").unwrap()
    });
    return RE.is_match_at(&permission, 0);
}

fn match_number_sequence(number: &String) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(0|[1-9][0-9]*)$))$").unwrap());
    return RE.is_match_at(&number, 0);
}

enum Permission {
    Add,
    Remove,
    ReadOne,
    ListAll,
    Read,
    Write,
    Assign,
    TripleGlob,
    DoubleGlob,
    SingleGlob,
    ID(LitInt),
    Name(LitStr),
}

impl Parse for Permission {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(token::TripleGlob) {
            input.parse::<token::TripleGlob>()?;
            return Ok(Permission::TripleGlob);
        }
        if input.peek(token::DoubleGlob) {
            input.parse::<token::DoubleGlob>()?;
            return Ok(Permission::DoubleGlob);
        }
        if input.peek(token::SingleGlob) {
            input.parse::<token::SingleGlob>()?;
            return Ok(Permission::SingleGlob);
        }
        if input.peek(token::add) {
            input.parse::<token::add>()?;
            return Ok(Permission::Add);
        }
        if input.peek(token::remove) {
            input.parse::<token::remove>()?;
            return Ok(Permission::Remove);
        }
        if input.peek(token::read_one) {
            input.parse::<token::read_one>()?;
            return Ok(Permission::ReadOne);
        }
        if input.peek(token::list_all) {
            input.parse::<token::list_all>()?;
            return Ok(Permission::ListAll);
        }
        if input.peek(token::read) {
            input.parse::<token::read>()?;
            return Ok(Permission::Read);
        }
        if input.peek(token::write) {
            input.parse::<token::write>()?;
            return Ok(Permission::Write);
        }
        if input.peek(token::assign) {
            input.parse::<token::assign>()?;
            return Ok(Permission::Assign);
        }
        if input.peek(LitInt) {
            let state = input.fork();
            let value: LitInt = state.parse()?;
            let str_content = value.to_string();
            if match_number_sequence(&str_content) && value.base10_parse::<i64>().is_ok() {
                input.parse::<LitInt>()?;
                // let value = value.base10_parse::<i64>()?;
                return Ok(Permission::ID(value));
            }
        }
        return Ok(Permission::Name(input.parse()?));
    }
}

pub struct Permissions {
    identifier: Punctuated<Permission, Token![.]>,
}

impl Parse for Permissions {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let permissions = syn::punctuated::Punctuated::parse_separated_nonempty(&input)?;
        return Ok(Permissions {
            identifier: permissions,
        });
    }
}

#[derive(Debug)]
pub enum PermissionParseError {
    Syn(syn::Error),
    InvalidOutput(String),
}

impl From<syn::Error> for PermissionParseError {
    fn from(err: syn::Error) -> Self {
        Self::Syn(err)
    }
}

impl From<String> for PermissionParseError {
    fn from(err: String) -> Self {
        Self::InvalidOutput(err)
    }
}

pub fn token_converter(permissions: Permissions) -> Result<Vec<tokenizer::Field>, PermissionParseError> {
    let parse_result: Result<Vec<tokenizer::Field>, PermissionParseError> = permissions
        .identifier
        .iter()
        .map(|permission| {
            Ok(match permission {
                Permission::ID(i) => tokenizer::Field::ID {
                    id: i.base10_parse::<i64>()?,
                },
                Permission::Name(name) => tokenizer::Field::Name { name: name.value() },
                Permission::Add => tokenizer::ListSpecifier::Add.into(),
                Permission::Remove => tokenizer::ListSpecifier::Remove.into(),
                Permission::ReadOne => tokenizer::ListSpecifier::ReadOne.into(),
                Permission::ListAll => tokenizer::ListSpecifier::ListAll.into(),
                Permission::Read => tokenizer::Specifier::Read.into(),
                Permission::Write => tokenizer::Specifier::Write.into(),
                Permission::Assign => tokenizer::Specifier::Assign.into(),
                Permission::SingleGlob => tokenizer::Field::Glob,
                Permission::DoubleGlob => tokenizer::Field::DoubleGlob,
                Permission::TripleGlob => tokenizer::Field::TripleGlob,
            })
        })
        .collect();

    return Ok(parse_result?);
}

fn parse_internal(permission: &String) -> Result<Vec<tokenizer::Field>, PermissionParseError> {
    if !match_strings(permission) {
        return Err(PermissionParseError::InvalidOutput(
            "The given permission string does not match the required format".to_string(),
        ));
    }
    let result: Permissions = syn::parse_str(&permission)?;

    return token_converter(result);
}

pub struct PermissionItem {
    pub perm: Vec<tokenizer::Field>,
}

impl Clone for PermissionItem {
    fn clone(&self) -> Self {
        Self {
            perm: self.perm.iter().map(|i| (*i).clone()).collect(),
        }
    }
}

impl From<Vec<tokenizer::Field>> for PermissionItem {
    fn from(value: Vec<tokenizer::Field>) -> Self {
        Self { perm: value }
    }
}

pub struct PermissionGroup {
    pub perms: Vec<PermissionItem>,
}

impl From<Vec<PermissionItem>> for PermissionGroup {
    fn from(value: Vec<PermissionItem>) -> Self {
        Self { perms: value }
    }
}

impl PermissionGroup {
    pub fn add(&mut self, item: PermissionItem) -> () {
        self.perms.push(item);
    }
}

impl Clone for PermissionGroup {
    fn clone(&self) -> Self {
        Self {
            perms: self.perms.iter().map(|i| (*i).clone()).collect(),
        }
    }
}

pub fn parse(permission: &String) -> Result<PermissionItem, PermissionParseError> {
    Ok(parse_internal(permission)?.into())
}