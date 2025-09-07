# permission_check
A relatively flexible permission checking library written in rust which checks for permission is scope (like: org.1028.user.*.write), with macro for compile time type checking and code generation

### How does the permission system work?
The permission system is scoped, and seperated with `.` with a few rules
- There are name, ID, specifier and glob in the permission system
- There are a list of specifier here
    - `read`
    - `write`
    - `assign`
    - `enact`
    - `add`
    - `remove`
    - `read_one`
    - `list_all`
- You can use multiple kind of globbing
    - `*` - a single scope level with any name or id (exclude specifier)
    - `**` - a single scope level with any name, id **or** specifier
    - `***` - **any scope level** of any name, id or specifier
- ID is just a literal positive integer (cannot be prefix with `0` unless it is just `0`)
- name is sequence of string made of uppercase, lowercase, number and underscore, with the first character not being number
    - `2x` - not allowed since it start with number
    - `x2` - allowed
- When you use a specifier, you cannot add more scope after it
    - `org.1.read` - allowed since the only specifier `read` is at the end
    - `org.1` - allowed since a specifier is not required, just most be at the end if included
    - `org.1.read.2` - **NOT ALLOWED**
- You cannot put 2 ID at once like `org.1.2`
- You can only use `***` glob level between 0 or 1 time
    - If `***` isn't used, it would imply `***` at the end

For the permission to match the requirement, the permission must perfectly encapsulate all specified requirement (which therefore, if globbing is used in the requirement, it would only be true **IF** the corresponding globbing is qualified in the permission)

You can use `check_expr` to check a more complex permission required, where you can use `|`, `&`, `^`, `!` and `()` to define what is required (check usage in `src/tests/test_expr.rs`), and you can use `permission_macro` to build-time permission checking (as seen from below) and variable encapsulation

Example:
```rs
use permission_macro::{perm_parser, perm_expr};

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
    assert_eq!(
        checker.with_perm(vec![perm_parser!(org.1047.user.243.read)]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.1047.user.243.read),
            perm_parser!(org.1047.user.243.write)
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.1047.user.write),
            perm_parser!(org.1047.user.read)
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.1047.user.write),
            perm_parser!(org.1047.user.read_one)
        ]),
        true
    );
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.1047.user.write),
            perm_parser!(org.1047.user.assign)
        ]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.1047.role.owner),
            perm_parser!(user.blacklist.enact)
        ]),
        false
    );
    assert_eq!(
        checker.with_perm(vec![
            perm_parser!(org.1047.role.owner),
            perm_parser!(user.blacklist.***)
        ]),
        true
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


### Every crate link
[permission_check](https://crates.io/crates/permission_check)

[permission_parser](https://crates.io/crates/permission_parser)

[permission_macro](https://crates.io/crates/permission_macro)


### Video demo

https://github.com/user-attachments/assets/ef21806e-9803-47a6-8e62-3a334087049a
