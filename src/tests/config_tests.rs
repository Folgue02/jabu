use crate::config::DependencySpec;

#[test]
fn parse_dependency_spec() {
    let input = "me.user-registry-0.0.1";
    let expected = DependencySpec::new("me.user", "registry", "0.0.1");

    assert_eq!(expected, DependencySpec::try_from(input).unwrap())
}

#[test]
fn parse_dependency_spec_with_hyphens() {
    let input = "me.user-registry-0.0.1-SNAPSHOT";
    let expected = DependencySpec::new("me.user", "registry", "0.0.1-SNAPSHOT");
    assert_eq!(expected, DependencySpec::try_from(input).unwrap())
}

#[test]
fn parse_invalid_dependency_spec() {
    let input = "me.userregistry-0.0.1SNAPSHOT";
    assert!(DependencySpec::try_from(input).is_err())
}
