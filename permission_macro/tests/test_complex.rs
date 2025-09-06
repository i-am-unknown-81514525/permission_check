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
    let checker = perm_expr!(
        ((org.1047.role.admin.enact | org.1047.role.owner.enact) | (org.1047.user.write && (org.1047.user.read | org.1047.user.read_one)) | (org.1047.user.243.read && org.1047.user.243.write)) &
        !(user.blacklist.enact & !user.blacklist.*)
    );
    assert_eq!(checker.with_perm(perm_parser!(org.1047.user.243)), true);
    assert_eq!(checker.with_perm(perm_parser!(org.1048.user.243)), false);
    assert_eq!(checker.with_perm(perm_parser!(org.1047.user.244)), false);
    assert_eq!(checker.with_perm(perm_parser!(org.1047.role.owner)), true);
    assert_eq!(checker.with_perm(perm_parser!(org.1048.role.owner)), false);
    assert_eq!(checker.with_perm(perm_parser!(org)), true);
    assert_eq!(checker.with_perm(perm_parser!(*)), true);
    assert_eq!(checker.with_perm(perm_parser!(org.1047)), true);
    assert_eq!(checker.with_perm(perm_parser!(org.1048)), false);
    assert_eq!(checker.with_perm(vec![perm_parser!(org.1047.user.243.read)]), false);
    assert_eq!(checker.with_perm(vec![perm_parser!(org.1047.user.243.read), perm_parser!(org.1047.user.243.write)]), true);
    assert_eq!(checker.with_perm(vec![perm_parser!(org.1047.user.write), perm_parser!(org.1047.user.read)]), true);
    assert_eq!(checker.with_perm(vec![perm_parser!(org.1047.user.write), perm_parser!(org.1047.user.read_one)]), true);
    assert_eq!(checker.with_perm(vec![perm_parser!(org.1047.user.write), perm_parser!(org.1047.user.assign)]), false);
    assert_eq!(checker.with_perm(vec![perm_parser!(org.1047.role.owner), perm_parser!(user.blacklist.enact)]), false);
    assert_eq!(checker.with_perm(vec![perm_parser!(org.1047.role.owner), perm_parser!(user.blacklist.***)]), true);
}