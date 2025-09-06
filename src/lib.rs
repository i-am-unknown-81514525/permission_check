use permission_parser::{tokenizer, ItemExpr, PermissionGroup, PermissionItem};

pub fn check_one(require: &PermissionItem, permission: &PermissionItem) -> bool {
    let mut idx_left = 0;
    let mut idx_right = 0;
    let size_left = require.perm.len();
    let size_right = permission.perm.len();
    let mut match_left_triple_glob: bool = false;
    let mut match_right_triple_glob: bool = false;
    loop {
        if idx_left == size_left || idx_right == size_right {
            if idx_left == size_left && idx_right != size_right {
                return false;
            }
            if idx_left == size_left && idx_right == size_right {
                return true;
            }
            if idx_left != size_left && idx_right == size_right {
                return true; // implicit *** applied for now, like org.1 perm mean org.1.user.2 is valid
                // If [***] is used, that is given as the anchor point and therefore they would always have same remaining length
            }
            break;
        }
        if match_left_triple_glob && match_right_triple_glob {
            let unprocessed_left = size_left - idx_left;
            let unprocessed_right = size_right - idx_right;
            if unprocessed_left == unprocessed_right {
                match_left_triple_glob = false;
                match_right_triple_glob = false;
            }
            if unprocessed_left > unprocessed_right {
                match_left_triple_glob = false;
            }
            if unprocessed_right > unprocessed_left {
                match_right_triple_glob = false;
            }
        }
        let field_required = require.perm[idx_left].clone();
        let field_permission = permission.perm[idx_right].clone();
        if match_left_triple_glob {
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
        match (field_required, field_permission, match_right_triple_glob) {
            (tokenizer::Field::TripleGlob, tokenizer::Field::TripleGlob, _) => {
                match_left_triple_glob = true;
                match_right_triple_glob = true;
            }
            (tokenizer::Field::TripleGlob, _, true) => {
                match_left_triple_glob = true;
            },
            (tokenizer::Field::TripleGlob, tokenizer::Field::DoubleGlob, false) => {},
            (tokenizer::Field::TripleGlob, _, false) => {
                return false;
            }
            (_, tokenizer::Field::TripleGlob, _) => {
                match_right_triple_glob = true;
            }
            (_, _, true) => {}
            (_, tokenizer::Field::DoubleGlob, _) => {}
            (tokenizer::Field::DoubleGlob, _, false) => {
                return false;
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
            (tokenizer::Field::ID { id: _ }, tokenizer::Field::Glob, _)
            | (tokenizer::Field::Name { name: _ }, tokenizer::Field::Glob, _)
            | (tokenizer::Field::Glob, tokenizer::Field::Glob, _) => {}
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

            (_, _, _) => {
                return false;
            }
        }
        if size_left - idx_left == size_right - idx_right {
            // [***].32
            // [***].32
            match_left_triple_glob = false;
            match_right_triple_glob = false;
        }
    }
    if match_left_triple_glob {
        return false;
    }
    if match_right_triple_glob {
        return true;
    }
    return true;
}

pub fn check(require: &PermissionItem, permissions: &PermissionGroup) -> bool {
    permissions.perms.iter().any(|p| check_one(require, p))
}

fn check_expr(expr: &ItemExpr, permissions: &PermissionGroup) -> bool {
    match expr {
        ItemExpr::Permission(p) => check(p, permissions),
        ItemExpr::And(l, r) => check_expr(l, permissions) && check_expr(r, permissions),
        ItemExpr::Or(l, r) => check_expr(l, permissions) || check_expr(r, permissions),
        ItemExpr::Not(e) => !check_expr(e, permissions),
        ItemExpr::Xor(l, r) => check_expr(l, permissions) ^ check_expr(r, permissions),
        ItemExpr::Bracketed(b) => check_expr(b, permissions)
    }
}

pub struct ComplexCheck {
    check_fn: Box<dyn Fn(&PermissionGroup) -> bool>
}

impl ComplexCheck {
    pub fn new(check_fn: Box<dyn Fn(&PermissionGroup) -> bool>) -> Self {
        Self {
            check_fn
        }
    }

    pub fn with_perm(&self, group: impl Into::<PermissionGroup>) -> bool {
        // Into::<Fn(&PermissionGroup) -> bool>::into(self.check_fn)(group.into())
        (self.check_fn)(&group.into()) 
    }

    pub fn from(expr: &ItemExpr) -> Self {
        let expr = expr.clone();
        Self {
            check_fn: Box::new(move |group| check_expr(&expr, group))
        }
    }
}
