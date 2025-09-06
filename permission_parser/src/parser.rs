
use crate::{token, tokenizer::{self}, Expr};
use regex::Regex;
use std::{sync::LazyLock};
use syn::{
    ext::IdentExt, parse::{Parse, ParseStream}, parse_str, punctuated::Punctuated, spanned::Spanned, Ident, Lit, LitInt, Token
};
use proc_macro2::Span;

// Part for Field exclude triple glob
// ([a-zA-Z0-9]+|\*|\*\*)
// (\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))*|(([a-zA-Z0-9]+|\*|\*\*)\.)*(\*\*\*|([a-zA-Z0-9]+|\*|\*\*))|(([a-zA-Z0-9]+|\*|\*\*)\.)+\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))+)

// fn match_strings(permission: &String) -> bool {
//     static RE: LazyLock<Regex> = LazyLock::new(|| {
//         Regex::new(r"^(\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))*|(([a-zA-Z0-9]+|\*|\*\*)\.)*(\*\*\*|([a-zA-Z0-9]+|\*|\*\*))|(([a-zA-Z0-9]+|\*|\*\*)\.)+\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))+)$").unwrap()
//     });
//     return RE.is_match_at(&permission, 0);
// }

fn match_number_sequence(number: &String) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(0|[1-9][0-9]*)$").unwrap());
    return RE.is_match_at(&number, 0);
}

#[derive(Clone)]
pub enum Permission {
    Add(Span),
    Remove(Span),
    ReadOne(Span),
    ListAll(Span),
    Read(Span),
    Write(Span),
    Assign(Span),
    Enact(Span),
    TripleGlob(Span),
    DoubleGlob(Span),
    SingleGlob(Span),
    ID(Span, LitInt),
    Name(Span, String),
}

impl Permission {
    pub fn span(&self) -> Span {
        return match self {
            Permission::Add(span) => *span,
            Permission::Remove(span) => *span,
            Permission::ReadOne(span) => *span,
            Permission::ListAll(span) => *span,
            Permission::Read(span) => *span,
            Permission::Write(span) => *span,
            Permission::Assign(span) => *span,
            Permission::TripleGlob(span) => *span,
            Permission::DoubleGlob(span) => *span,
            Permission::SingleGlob(span) => *span,
            Permission::ID(span, _) => *span,
            Permission::Name(span, _) => *span,
            Permission::Enact(span) => *span
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            Permission::Add(_) => "add",
            Permission::Remove(_) => "remove",
            Permission::ReadOne(_) => "read_one",
            Permission::ListAll(_) => "list_all",
            Permission::Read(_) => "read",
            Permission::Write(_) => "write",
            Permission::Assign(_) => "assign",
            Permission::Enact(_) => "enact",
            Permission::TripleGlob(_) => "***",
            Permission::DoubleGlob(_) => "**",
            Permission::SingleGlob(_) => "*",
            Permission::ID(_, _) => "custom_id",
            Permission::Name(_, _) => "custom_name",
        }
    }
}

impl Parse for Permission {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(token::TripleGlob) {
            let r = input.parse::<token::TripleGlob>()?;
            return Ok(Permission::TripleGlob(r.span()));
        }
        if input.peek(token::DoubleGlob) {
            let r = input.parse::<token::DoubleGlob>()?;
            return Ok(Permission::DoubleGlob(r.span()));
        }
        if input.peek(token::SingleGlob) {
            let r = input.parse::<token::SingleGlob>()?;
            return Ok(Permission::SingleGlob(r.span()));
        }
        if input.peek(token::add) {
            let r = input.parse::<token::add>()?;
            return Ok(Permission::Add(r.span()));
        }
        if input.peek(token::remove) {
            let r = input.parse::<token::remove>()?;
            return Ok(Permission::Remove(r.span()));
        }
        if input.peek(token::read_one) {
            let r = input.parse::<token::read_one>()?;
            return Ok(Permission::ReadOne(r.span()));
        }
        if input.peek(token::list_all) {
            let r = input.parse::<token::list_all>()?;
            return Ok(Permission::ListAll(r.span()));
        }
        if input.peek(token::read) {
            let r = input.parse::<token::read>()?;
            return Ok(Permission::Read(r.span()));
        }
        if input.peek(token::write) {
            let r = input.parse::<token::write>()?;
            return Ok(Permission::Write(r.span()));
        }
        if input.peek(token::assign) {
            let r = input.parse::<token::assign>()?;
            return Ok(Permission::Assign(r.span()));
        }
        if input.peek(token::enact) {
            let r = input.parse::<token::enact>()?;
            return Ok(Permission::Assign(r.span()));
        }
        if input.peek(LitInt) {
            let state = input.fork();
            let value: LitInt = state.parse()?;
            let str_content = value.to_string();
            if match_number_sequence(&str_content) && value.base10_parse::<i64>().is_ok() {
                input.parse::<LitInt>()?;
                // let value = value.base10_parse::<i64>()?;
                return Ok(Permission::ID(value.span(), value));
            }
        }
        if input.peek(Lit) {
            let value: Lit = input.parse()?;
            return Ok(
                Permission::Name(
                    value.span(), 
                    match value.clone() {
                        Lit::Str(s) => s.value(),
                        Lit::Bool(b) => b.value.to_string(),
                        Lit::Float(_) => return Err(syn::Error::new(value.span(), "2 ID cannot appear consecutively")),
                        Lit::Int(_) => unreachable!(),
                        Lit::Byte(_) => return Err(syn::Error::new(value.span(), "Invalid syntax: use of byte is not allowed")),
                        Lit::Char(c) => c.value().to_string(),
                        Lit::ByteStr(_) => return Err(syn::Error::new(value.span(), "Invalid syntax: use of byte string is not allowed")),
                        Lit::CStr(_) => return Err(syn::Error::new(value.span(), "Invalid syntax: use of Cstr is not allowed")),
                        Lit::Verbatim(v) => v.to_string(),
                        _ => unreachable!()
                    }
                )
            );
        }
        // let value: Ident = input.parse()?;
        let value = input.call(Ident::parse_any)?;
        return Ok(Permission::Name(value.span(), value.to_string()));
    }
}

#[derive(Clone)]
pub struct Permissions {
    pub identifier: Punctuated<Permission, Token![.]>,
}

enum Terminator {
    ListSpecifier(Permission, Span),
    Specifier(Permission, Span)
}

impl Parse for Permissions {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let permissions = syn::punctuated::Punctuated::parse_separated_nonempty(&input)?;
        let mut triple_glob_count: i32 = 0;
        let mut is_terminated: Option<Terminator> = None;
        for item in &permissions {
            if is_terminated.is_some() {
                return Err(match is_terminated {
                    Some(Terminator::ListSpecifier(specifier, span)) => 
                        syn::Error::new(
                            *specifier.span().join(span).get_or_insert(specifier.span()), 
                            format!("Cannot use further define the permission after using list specifier (`{}`)", specifier.name()
                        )
                    ),
                    Some(Terminator::Specifier(specifier, span, )) => 
                        syn::Error::new(
                            *specifier.span().join(span).get_or_insert(specifier.span()), 
                            format!("Cannot use further define the permission after using specifier (`{}`)", specifier.name()
                        )
                    ),
                    None => unreachable!()
                })
            }
            match item {
                Permission::TripleGlob(span) => {
                    if triple_glob_count > 0 {
                        return Err(syn::Error::new(*span, "Cannot use triple glob more than once in a permission"));
                    }
                    triple_glob_count += 1;
                }
                Permission::Name(_, _) | 
                Permission::ID(_, _) | 
                Permission::SingleGlob(_) | 
                Permission::DoubleGlob(_) => {}
                specifier @ _ => {
                    is_terminated = match specifier {
                        Permission::Add(span) | Permission::Remove(span) | Permission::ReadOne(span) | Permission::ListAll(span) => 
                            Some(Terminator::ListSpecifier((*specifier).clone(), *span)),
                        _ => Some(Terminator::Specifier((*specifier).clone(), specifier.span()))
                    }
                }
            }
        }
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
                Permission::ID(_, i) => tokenizer::Field::ID {
                    id: i.base10_parse::<i64>()?,
                },
                Permission::Name(_, name) => tokenizer::Field::Name { name: name.to_string() },
                Permission::Add(_) => tokenizer::ListSpecifier::Add.into(),
                Permission::Remove(_) => tokenizer::ListSpecifier::Remove.into(),
                Permission::ReadOne(_) => tokenizer::ListSpecifier::ReadOne.into(),
                Permission::ListAll(_) => tokenizer::ListSpecifier::ListAll.into(),
                Permission::Read(_) => tokenizer::Specifier::Read.into(),
                Permission::Write(_) => tokenizer::Specifier::Write.into(),
                Permission::Assign(_) => tokenizer::Specifier::Assign.into(),
                Permission::Enact(_) => tokenizer::Specifier::Enact.into(),
                Permission::SingleGlob(_) => tokenizer::Field::Glob,
                Permission::DoubleGlob(_) => tokenizer::Field::DoubleGlob,
                Permission::TripleGlob(_) => tokenizer::Field::TripleGlob,
            })
        })
        .collect();

    return Ok(parse_result?);
}

fn parse_internal(permission: &String) -> Result<Vec<tokenizer::Field>, PermissionParseError> {
    // if !match_strings(permission) {
    //     return Err(PermissionParseError::InvalidOutput(
    //         "The given permission string does not match the required format".to_string(),
    //     ));
    // }
    let result: Permissions = syn::parse_str(&permission)?;

    return token_converter(result);
}

#[cfg_attr(debug_assertions, derive(Debug))]
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

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PermissionGroup {
    pub perms: Vec<PermissionItem>,
}

impl From<&PermissionItem> for PermissionGroup {
    fn from(value: &PermissionItem) -> Self {
        Self { perms: vec![value.clone()] }
    }
}

impl From<PermissionItem> for PermissionGroup {
    fn from(value: PermissionItem) -> Self {
        Self { perms: vec![value] }
    }
}

impl From<Vec<PermissionItem>> for PermissionGroup {
    fn from(value: Vec<PermissionItem>) -> Self {
        Self { perms: value }
    }
}

impl From<&PermissionGroup> for PermissionGroup {
    fn from(value: &PermissionGroup) -> PermissionGroup {
        value.clone()
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

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ItemExpr {
    Permission(PermissionItem),
    Not(Box<ItemExpr>),
    And(Box<ItemExpr>, Box<ItemExpr>),
    Or(Box<ItemExpr>, Box<ItemExpr>),
    Xor(Box<ItemExpr>, Box<ItemExpr>),
    Bracketed(Box<ItemExpr>)
}

impl ItemExpr {
    pub fn from_expr(item: Expr) -> Result<Self, PermissionParseError> {
        Ok(
            match item {
                Expr::Permission(p) => Self::Permission(
                    PermissionItem {perm: token_converter(p)?}
                ),
                Expr::Not(n) => Self::Not(Box::new(Self::from_expr(*n)?)),
                Expr::And(l, r) => Self::And(Box::new(Self::from_expr(*l)?), Box::new(Self::from_expr(*r)?)),
                Expr::Or(l, r) => Self::Or(Box::new(Self::from_expr(*l)?), Box::new(Self::from_expr(*r)?)),
                Expr::Xor(l, r) => Self::Xor(Box::new(Self::from_expr(*l)?), Box::new(Self::from_expr(*r)?)),
                Expr::Bracketed(b) => Self::Bracketed(Box::new(Self::from_expr(*b)?))
            }
        )
    }
}


pub fn expr_parse(expr: &String) -> Result<ItemExpr, PermissionParseError> {
    let result: Expr = parse_str(expr)?;
    return ItemExpr::from_expr(result);
}

#[test]
fn test_parse() {
    parse(&"a.b.c.***.d".to_string()).unwrap();
}