use permission_check::check_one;
use permission_parser::parse;

#[test]
fn require_same() {
    let perm1 = parse("perm.1").unwrap();
    let perm2 = parse("perm.1").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn require_same_length_diff() {
    let perm1 = parse("perm.2").unwrap();
    let perm2 = parse("perm.1").unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn require_more_specify() {
    let perm1 = parse("perm.1.inner.add").unwrap();
    let perm2 = parse("perm.1").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn require_less_specify() {
    let perm1 = parse("perm.1").unwrap();
    let perm2 = parse("perm.1.inner").unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn one_glob() {
    let perm1 = parse("perm.1.inner").unwrap();
    let perm2 = parse("perm.*.inner").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn one_glob_inverse() {
    let perm1 = parse("perm.*.inner").unwrap();
    let perm2 = parse("perm.1.inner").unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn double_glob() {
    let perm1 = parse("perm.add").unwrap();
    let perm2 = parse("perm.**").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn single_glob_no_identifier() {
    let perm1 = parse("perm.add").unwrap();
    let perm2 = parse("perm.*").unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn triple_glob_end() {
    let perm1 = parse("perm.**.abc.test").unwrap();
    let perm2 = parse("perm.***").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_mid() {
    let perm1 = parse("perm.**.abc.test").unwrap();
    let perm2 = parse("perm.***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_mid_double_end() {
    let perm1 = parse("perm.abc.**.test").unwrap();
    let perm2 = parse("perm.***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_mid_double_end_many_glob() {
    let perm1 = parse("perm.abc.**.**.**.test").unwrap();
    let perm2 = parse("perm.***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn triple_glob_start_double_end_many_glob() {
    let perm1 = parse("perm.abc.**.**.**.test").unwrap();
    let perm2 = parse("***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn both_triple_glob() {
    let perm1 = parse("perm.abc.***.test").unwrap();
    let perm2 = parse("***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn both_triple_glob_2() {
    let perm1 = parse("perm.abc.***.abc.test").unwrap();
    let perm2 = parse("***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn both_triple_glob_mismatch() {
    let perm1 = parse("perm.abc.***.abc.test.1").unwrap();
    let perm2 = parse("***.test.2").unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn both_triple_glob_mismatch_2() {
    let perm1 = parse("perm.***.abc.test").unwrap();
    let perm2 = parse("perm.abc.***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn both_triple_glob_not_mismatch() {
    let perm1 = parse("perm.***.abc.test").unwrap();
    let perm2 = parse("perm.**.***.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}

#[test]
fn diff_in_middle() {
    let perm1 = parse("perm.**.123.test").unwrap();
    let perm2 = parse("perm.**.124.test").unwrap();
    assert_eq!(check_one(&perm1, &perm2), false);
}

#[test]
fn both_triple_glob_match() {
    let perm1 = parse("perm.**.***.**.test.1").unwrap();
    let perm2 = parse("perm.***.test.1").unwrap();
    assert_eq!(check_one(&perm1, &perm2), true);
}
