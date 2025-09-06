use permission_macro::{perm_parser, perm_expr};

#[test]
fn test_generation() {

    perm_parser!(a.b.cd.***.b.1974.add);
    perm_parser!(a.inner.*.test);
}

#[test]
fn test_other() {
    perm_expr!(test.abc.1 | test.abc.2)(&perm_parser!(test.abc.1).into());
}