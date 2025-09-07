use permission_check::check_one;
use permission_parser::parse;

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

#[test]
fn require_more_specify() {
    let perm1 = parse(&"perm.1.inner.add".to_string()).unwrap();
    let perm2 = parse(&"perm.1".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn require_less_specify() {
    let perm1 = parse(&"perm.1".to_string()).unwrap();
    let perm2 = parse(&"perm.1.inner".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn one_glob() {
    let perm1 = parse(&"perm.1.inner".to_string()).unwrap();
    let perm2 = parse(&"perm.*.inner".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn one_glob_inverse() {
    let perm1 = parse(&"perm.*.inner".to_string()).unwrap();
    let perm2 = parse(&"perm.1.inner".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn double_glob() {
    let perm1 = parse(&"perm.add".to_string()).unwrap();
    let perm2 = parse(&"perm.**".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn single_glob_no_identifier() {
    let perm1 = parse(&"perm.add".to_string()).unwrap();
    let perm2 = parse(&"perm.*".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn triple_glob_end() {
    let perm1 = parse(&"perm.**.abc.test".to_string()).unwrap();
    let perm2 = parse(&"perm.***".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_mid() {
    let perm1 = parse(&"perm.**.abc.test".to_string()).unwrap();
    let perm2 = parse(&"perm.***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_mid_double_end() {
    let perm1 = parse(&"perm.abc.**.test".to_string()).unwrap();
    let perm2 = parse(&"perm.***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_mid_double_end_many_glob() {
    let perm1 = parse(&"perm.abc.**.**.**.test".to_string()).unwrap();
    let perm2 = parse(&"perm.***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_start_double_end_many_glob() {
    let perm1 = parse(&"perm.abc.**.**.**.test".to_string()).unwrap();
    let perm2 = parse(&"***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn both_triple_glob() {
    let perm1 = parse(&"perm.abc.***.test".to_string()).unwrap();
    let perm2 = parse(&"***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn both_triple_glob_2() {
    let perm1 = parse(&"perm.abc.***.abc.test".to_string()).unwrap();
    let perm2 = parse(&"***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn both_triple_glob_mismatch() {
    let perm1 = parse(&"perm.abc.***.abc.test.1".to_string()).unwrap();
    let perm2 = parse(&"***.test.2".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn both_triple_glob_mismatch_2() {
    let perm1 = parse(&"perm.***.abc.test".to_string()).unwrap();
    let perm2 = parse(&"perm.abc.***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn both_triple_glob_not_mismatch() {
    let perm1 = parse(&"perm.***.abc.test".to_string()).unwrap();
    let perm2 = parse(&"perm.**.***.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn diff_in_middle() {
    let perm1 = parse(&"perm.**.123.test".to_string()).unwrap();
    let perm2 = parse(&"perm.**.124.test".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn both_triple_glob_match() {
    let perm1 = parse(&"perm.**.***.**.test.1".to_string()).unwrap();
    let perm2 = parse(&"perm.***.test.1".to_string()).unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}
