use regex::{Regex, Match};
use std::sync::LazyLock;

pub enum Specifier {
    ListSpecifier {specifier: ListSpecifier},
    Read,
    Write,
    Assign // for T.assign, allow assign permission to anything at T
}

pub enum ListSpecifier {
    Add,
    Remove
}

pub enum Field {
    Name {name: String},
    ID {id: i64},
    Specifier {specifier: Specifier},
    Glob, // Qualify for Name, ID
    DoubleGlob, // Qualify for Name, ID and Specifier
    TripleGlob, // Qualify for Name, ID and Specifier for any length (can only appear once)
}

pub enum Token {
    Seperator,
    Field {field: Field}
}

// Part for Field exclude triple glob
// ([a-zA-Z0-9]+|\*|\*\*)
// (\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))*|(([a-zA-Z0-9]+|\*|\*\*)\.)*(\*\*\*|([a-zA-Z0-9]+|\*|\*\*))|(([a-zA-Z0-9]+|\*|\*\*)\.)+\*\*\*(\.([a-zA-Z0-9]+|\*|\*\*))+)

fn match_strings(permission: &String) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"").unwrap());
}

pub fn parse(permission: &String) -> Vec<Token> {

}