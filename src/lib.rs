use regex::Regex;
use std::{cmp::min, sync::LazyLock};
use syn::{
    LitInt, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

mod tokenizer {

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Specifier {
        ListSpecifier { specifier: ListSpecifier },
        Read,
        Write,
        Assign, // for T.assign, allow assign permission to anything at T
    }

    impl From<ListSpecifier> for Specifier {
        fn from(value: ListSpecifier) -> Self {
            Self::ListSpecifier { specifier: value }
        }
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ListSpecifier {
        Add,
        Remove,
        ReadOne,
        ListAll,
    }

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Field {
        Name { name: String },
        ID { id: i64 },
        Specifier { specifier: Specifier },
        Glob,       // Qualify for Name, ID
        DoubleGlob, // Qualify for Name, ID and Specifier
        TripleGlob, // Qualify for Name, ID and Specifier for any length (can only appear once)
    }

    impl From<ListSpecifier> for Field {
        fn from(value: ListSpecifier) -> Self {
            Self::Specifier {
                specifier: Specifier::ListSpecifier { specifier: value },
            }
        }
    }

    impl From<Specifier> for Field {
        fn from(value: Specifier) -> Self {
            Self::Specifier { specifier: value }
        }
    }
}

mod token {

    syn::custom_keyword!(add);
    syn::custom_keyword!(remove);
    syn::custom_keyword!(read_one);
    syn::custom_keyword!(list_all);
    syn::custom_keyword!(read);
    syn::custom_keyword!(write);
    syn::custom_keyword!(assign);

    syn::custom_punctuation!(SingleGlob, *);
    syn::custom_punctuation!(DoubleGlob, **);
    syn::custom_punctuation!(TripleGlob, ***);
}

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

struct Permissions {
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

fn parse_internal(permission: &String) -> Result<Vec<tokenizer::Field>, PermissionParseError> {
    if !match_strings(permission) {
        return Err(PermissionParseError::InvalidOutput(
            "The given permission string does not match the required format".to_string(),
        ));
    }
    let result: Permissions = syn::parse_str(&permission)?;

    let parse_result: Result<Vec<tokenizer::Field>, PermissionParseError> = result
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

pub struct PermissionItem {
    perm: Vec<tokenizer::Field>,
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
    perms: Vec<PermissionItem>,
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

pub fn check_one(require: PermissionItem, permission: PermissionItem) -> bool {
    let mut idx_left = 0;
    let mut idx_right = 0;
    let size_left = require.perm.len();
    let size_right = require.perm.len();
    let mut match_left_triple_glob: bool = false;
    let mut match_right_triple_glob: bool = false;
    loop {
        if (match_left_triple_glob && match_right_triple_glob) {
            let unprocessed_left = size_left - idx_left;
            let unprocessed_right = size_right - idx_right;
            if (unprocessed_left == unprocessed_right) {
                match_left_triple_glob = false;
                match_right_triple_glob = false;
            }
            if (unprocessed_left > unprocessed_right) {
                match_left_triple_glob = false;
            }
            if (unprocessed_right > unprocessed_left) {
                match_right_triple_glob = false;
            }
        }
        let field_required = require.perm[idx_left].clone();
        let field_permission = require.perm[idx_right].clone();
        if (match_left_triple_glob) {
            if field_permission != tokenizer::Field::DoubleGlob
                && field_permission != tokenizer::Field::TripleGlob
            {
                return false;
            }
        }
        if !match_left_triple_glob {
            idx_left += 1;
        }
        if !match_right_triple_glob {
            idx_right += 1;
        }
        if (idx_left == size_left || idx_right == size_right) {
            if (idx_left == size_left && idx_right != size_right) {
                return false;
            }
            if (idx_left == size_left && idx_right == size_right) {
                return true;
            }
            if (idx_left != size_left && idx_right == size_right) {
                return true; // implicit *** applied for now, like org.1 perm mean org.1.user.2 is valid
                // If [***] is used, that is given as the anchor point and therefore they would always have same remaining length
            }
            break;
        }
        if (size_left - idx_left == size_right - idx_right) {
            // [***].32
            // [***].32
            match_left_triple_glob = false;
            match_right_triple_glob = false;
        }
        match (field_required, field_permission, match_right_triple_glob) {
            (tokenizer::Field::TripleGlob, tokenizer::Field::TripleGlob, _) => {
                match_left_triple_glob = true;
                match_right_triple_glob = true;
            }
            (tokenizer::Field::TripleGlob, _, _) => {
                match_left_triple_glob = true;
            }
            (_, tokenizer::Field::TripleGlob, _) => {
                match_right_triple_glob = true;
            }
            (_, _, true) => {}
            (_, tokenizer::Field::DoubleGlob, _) => {}
            (tokenizer::Field::DoubleGlob, _, false) => {
                return false;
            }
            (_, tokenizer::Field::Glob, _) => {}
            (tokenizer::Field::Glob, _, false) => {
                return false;
            }
            (tokenizer::Field::ID { id: lid }, tokenizer::Field::ID { id: rid }, false) => {
                if lid != rid {
                    return false;
                };
            }
            (
                tokenizer::Field::Name { name: lname },
                tokenizer::Field::Name { name: rname },
                false,
            ) => {
                if lname != rname {
                    return false;
                };
            }
            (
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::Add,
                        },
                },
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::Add,
                        },
                },
                _,
            ) => {}
            (
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::Remove,
                        },
                },
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::Remove,
                        },
                },
                _,
            ) => {}
            (
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::ReadOne,
                        },
                },
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::ReadOne,
                        },
                },
                _,
            ) => {}
            (
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::ListAll,
                        },
                },
                tokenizer::Field::Specifier {
                    specifier:
                        tokenizer::Specifier::ListSpecifier {
                            specifier: tokenizer::ListSpecifier::ListAll,
                        },
                },
                _,
            ) => {}
            (
                tokenizer::Field::Specifier {
                    specifier: tokenizer::Specifier::Assign,
                },
                tokenizer::Field::Specifier {
                    specifier: tokenizer::Specifier::Assign,
                },
                _,
            ) => {}
            (
                tokenizer::Field::Specifier {
                    specifier: tokenizer::Specifier::Read,
                },
                tokenizer::Field::Specifier {
                    specifier: tokenizer::Specifier::Read,
                },
                _,
            ) => {}
            (
                tokenizer::Field::Specifier {
                    specifier: tokenizer::Specifier::Write,
                },
                tokenizer::Field::Specifier {
                    specifier: tokenizer::Specifier::Write,
                },
                _,
            ) => {}
            (_, _, _) => {
                return false;
            }
        }
    }
    if (match_left_triple_glob) {
        return false;
    }
    if (match_right_triple_glob) {
        return true;
    }
    return true;
}

pub fn check(require: PermissionItem, permissions: PermissionGroup) {}
