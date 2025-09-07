
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Specifier {
    ListSpecifier { specifier: ListSpecifier },
    Read,
    Write,
    Assign, // for T.assign, allow assign permission to anything at T
    Enact
}

impl ToString for Specifier {
    fn to_string(&self) -> String {
        match self {
            Specifier::ListSpecifier { specifier } => specifier.to_string(),
            Specifier::Assign => "assign".to_string(),
            Specifier::Read => "read".to_string(),
            Specifier::Write => "write".to_string(),
            Specifier::Enact => "enact".to_string()
        }
    }
}

impl From<ListSpecifier> for Specifier {
    fn from(value: ListSpecifier) -> Self {
        Self::ListSpecifier { specifier: value }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ListSpecifier {
    Add,
    Remove,
    ReadOne,
    ListAll,
}

impl ToString for ListSpecifier {
    fn to_string(&self) -> String {
        match self {
            ListSpecifier::Add => "add".to_string(),
            ListSpecifier::ListAll => "list_all".to_string(),
            ListSpecifier::ReadOne => "read_one".to_string(),
            ListSpecifier::Remove => "remove".to_string()
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
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

impl ToString for Field {
    fn to_string(&self) -> String {
        match self {
            Field::Name { name } => name.clone(),
            Field::ID { id } => id.to_string(),
            Field::Specifier { specifier } => specifier.to_string(),
            Field::Glob => "*".to_string(),
            Field::DoubleGlob => "**".to_string(),
            Field::TripleGlob => "***".to_string()
        }
    }
}
