use permission_macro::{perm_expr, perm_parser};

#[test]
fn test_generation() {
    perm_parser!(a.b.cd.***.b.1974.add);
    perm_parser!(a.inner.*.test);
    perm_parser!(a.false);
    perm_parser!(false.true.pub);
    let x = "1";
    perm_parser!(test.{x});
    perm_expr!(test.{x});
    perm_expr!(test.{x});
    let y = "1".to_string();
    perm_parser!(test.{y});
    perm_expr!(test.{y}); // btw this wouls move `y` due to the unfortunate implementation of expr that I cannot fix easily
}

#[test]
fn test_other() {
    let result = perm_expr!(test.abc.1 | test.abc.2).with_perm(perm_parser!(test.abc.1));
    assert_eq!(result, true);
}
