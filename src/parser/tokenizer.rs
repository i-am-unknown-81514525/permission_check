
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
