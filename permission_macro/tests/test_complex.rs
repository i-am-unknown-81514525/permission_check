use permission_macro::{perm_parser, perm_expr};


#[test]
fn always_false() {
    let result = perm_expr!(test.abc.1 & !test.abc.1).with_perm(perm_parser!(test.abc.1));
    assert_eq!(result, false);
}

#[test]
fn always_true() {
    let result = perm_expr!(test.abc.1 | !test.abc.1).with_perm(perm_parser!(test.abc.1));
    assert_eq!(result, true);
}

#[test]
fn test_1() {
    let result = perm_expr!(
        org.1047.role.admin | (org.1047.user.write && (org.1047.user.read | org.1047.user.read_one)) | (org.1047.user.243.read && org.1047.user.243.write)
    ).with_perm(perm_parser!(org.1047.user.243));
    assert_eq!(result, true);
}