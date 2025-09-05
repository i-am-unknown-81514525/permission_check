use permission_check::check_one;
use permission_parser::{parse};

#[test]
fn require_same() {
    let perm1 = parse(&"perm.1".to_string()).unwrap();
    let perm2 = parse(&"perm.1".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn require_same_length_diff() {
    let perm1 = parse(&"perm.2".to_string()).unwrap();
    let perm2 = parse(&"perm.1".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}