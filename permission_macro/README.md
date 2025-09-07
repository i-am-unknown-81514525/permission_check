# permission_macro

A sub-crate of `permission_check`, which is used to provide more extensive support for build-time checking of permissions.

### A simple example
```rs
use permission_macro::{perm_parser, perm_expr};

#[test]
fn test_1() {
    let checker = perm_expr!(
        ((org.1047.role.admin.enact | org.1047.role.owner.enact) | (org.1047.user.write && (org.1047.user.read | org.1047.user.read_one)) | (org.1047.user.243.read && org.1047.user.243.write)) &
        !(user.blacklist.enact & !user.blacklist.*)
    );
    let org_id = 1047;
    let user_id = 243;
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.{org_id}.user.{user_id}.read),
            perm_parser!(org.{org_id}.user.{user_id}.write)
        ]),
        true
    );
    let org_id = 1047;
    let user_id = 244;
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.{org_id}.user.{user_id}.read),
            perm_parser!(org.{org_id}.user.{user_id}.write)
        ]),
        false
    );
}
```

For better example, check the README in `permission_check` crate


### Every crate link
[permission_check](https://crates.io/crates/permission_check)

[permission_parser](https://crates.io/crates/permission_parser)

[permission_macro](https://crates.io/crates/permission_macro)