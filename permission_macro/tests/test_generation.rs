use permission_macro::perm_parser;

#[test]
fn test_generation() {

    perm_parser!(a.b.cd.***.b.1974.add);
}