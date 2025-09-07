use permission_check::ComplexCheck;
use permission_parser::{ItemExpr, PermissionParseError, expr_parse, parse};

#[test]
fn test_expr() -> Result<(), PermissionParseError> {
    let expr: ItemExpr = expr_parse(
        "(
            (org.1047.role.admin.enact | org.1047.role.owner.enact) | 
            (org.1047.user.write && (org.1047.user.read | org.1047.user.read_one)) | (org.1047.user.243.read && org.1047.user.243.write)
        ) & 
        !(user.blacklist.enact & !user.blacklist.*)").unwrap();
    let checker = ComplexCheck::from(&expr);
    assert_eq!(
        checker.with_perm(parse("org.1047.user.243")?),
        true
    );
    assert_eq!(
        checker.with_perm(parse("org.1048.user.243")?),
        false
    );
    assert_eq!(
        checker.with_perm(parse("org.1047.user.244")?),
        false
    );
    assert_eq!(
        checker.with_perm(parse("org.1047.role.owner")?),
        true
    );
    assert_eq!(
        checker.with_perm(parse("org.1048.role.owner")?),
        false
    );
    assert_eq!(checker.with_perm(parse("org")?), true);
    assert_eq!(checker.with_perm(parse("*")?), true);
    assert_eq!(checker.with_perm(parse("org.1047")?), true);
    assert_eq!(checker.with_perm(parse("org.1048")?), false);
    assert_eq!(
        checker.with_perm(vec![parse("org.1047.user.243.read")?]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            parse("org.1047.user.243.read")?,
            parse("org.1047.user.243.write")?
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            parse("org.1047.user.write")?,
            parse("org.1047.user.read")?
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            parse("org.1047.user.write")?,
            parse("org.1047.user.read_one")?
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            parse("org.1047.user.write")?,
            parse("org.1047.user.assign")?
        ]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            parse("org.1047.role.owner")?,
            parse("user.blacklist.enact")?
        ]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            parse("org.1047.role.owner")?,
            parse("user.blacklist.***")?
        ]),
        true
    );
    Ok(())
}

#[test]
fn test_var_kind_err() {
    assert_eq!(parse("user.blacklist.***").is_err(), false);
    assert_eq!(parse("user.blacklist.{user_id}").is_err(), true);
    assert_eq!(expr_parse(
        "(
            (org.1047.role.admin.enact | org.1047.role.owner.enact) | 
            (org.1047.user.write && (org.1047.user.read | org.1047.user.read_one)) | (org.1047.user.243.read && org.1047.user.243.write)
        ) & 
        !(user.blacklist.enact & !user.blacklist.{user_id})").is_err(), true);
}
