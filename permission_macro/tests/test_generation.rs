use permission_macro::{perm_parser, perm_expr};

#[test]
fn test_generation() {

    perm_parser!(a.b.cd.***.b.1974.add);
    perm_parser!(a.inner.*.test);
    perm_parser!(a.false);
    perm_parser!(false.true.pub);
}

#[test]
fn test_other() {
    let result = perm_expr!(test.abc.1 | test.abc.2).with_perm(perm_parser!(test.abc.1));
    assert_eq!(result, true);
}