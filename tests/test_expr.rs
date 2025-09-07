use permission_check::ComplexCheck;
use permission_parser::{ItemExpr, PermissionParseError, expr_parse, parse};

#[test]
fn test_expr() -> Result<(), PermissionParseError> {
    let expr: ItemExpr = expr_parse(
        &"((org.1047.role.admin.enact | org.1047.role.owner.enact) | (org.1047.user.write && (org.1047.user.read | org.1047.user.read_one)) | (org.1047.user.243.read && org.1047.user.243.write)) &
        !(user.blacklist.enact & !user.blacklist.*)".to_string()).unwrap();
    let checker = ComplexCheck::from(&expr);
    assert_eq!(
        checker.with_perm(parse(&"org.1047.user.243".to_string())?),
        true
    );
    assert_eq!(
        checker.with_perm(parse(&"org.1048.user.243".to_string())?),
        false
    );
    assert_eq!(
        checker.with_perm(parse(&"org.1047.user.244".to_string())?),
        false
    );
    assert_eq!(
        checker.with_perm(parse(&"org.1047.role.owner".to_string())?),
        true
    );
    assert_eq!(
        checker.with_perm(parse(&"org.1048.role.owner".to_string())?),
        false
    );
    assert_eq!(checker.with_perm(parse(&"org".to_string())?), true);
    assert_eq!(checker.with_perm(parse(&"*".to_string())?), true);
    assert_eq!(checker.with_perm(parse(&"org.1047".to_string())?), true);
    assert_eq!(checker.with_perm(parse(&"org.1048".to_string())?), false);
    assert_eq!(
        checker.with_perm(vec![parse(&"org.1047.user.243.read".to_string())?]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            parse(&"org.1047.user.243.read".to_string())?,
            parse(&"org.1047.user.243.write".to_string())?
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            parse(&"org.1047.user.write".to_string())?,
            parse(&"org.1047.user.read".to_string())?
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            parse(&"org.1047.user.write".to_string())?,
            parse(&"org.1047.user.read_one".to_string())?
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            parse(&"org.1047.user.write".to_string())?,
            parse(&"org.1047.user.assign".to_string())?
        ]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            parse(&"org.1047.role.owner".to_string())?,
            parse(&"user.blacklist.enact".to_string())?
        ]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            parse(&"org.1047.role.owner".to_string())?,
            parse(&"user.blacklist.***".to_string())?
        ]),
        true
    );
    Ok(())
}

#[test]
fn test_var_kind_err() {
    assert_eq!(parse(&"user.blacklist.***".to_string()).is_err(), false);
    assert_eq!(parse(&"user.blacklist.{user_id}".to_string()).is_err(), true);
    assert_eq!(expr_parse(
        &"((org.1047.role.admin.enact | org.1047.role.owner.enact) | (org.1047.user.write && (org.1047.user.read | org.1047.user.read_one)) | (org.1047.user.243.read && org.1047.user.243.write)) &
        !(user.blacklist.enact & !user.blacklist.{user_id})".to_string()).is_err(), true);
}
