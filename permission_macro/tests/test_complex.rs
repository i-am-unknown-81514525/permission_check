use permission_macro::{perm_parser, perm_expr};


#[test]
fn always_false() {
    let result = perm_expr!(test.abc.1 & !test.abc.1)(&perm_parser!(test.abc.1).into());
    assert_eq!(result, false);
}

#[test]
fn always_true() {
    let result = perm_expr!(test.abc.1 | !test.abc.1)(&perm_parser!(test.abc.1).into());
    assert_eq!(result, true);
}