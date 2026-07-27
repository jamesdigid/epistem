use epistem::manifest::ManifestValidator;

#[test]
fn validates_manifest() {
    let validator = ManifestValidator::default();
    let report = validator.validate_path("examples/gmail-send");

    assert!(report.valid, "{:#?}", report.issues);
    let manifest = report.manifest.expect("manifest should be present");
    assert_eq!(manifest.name, "gmail-send");
    assert_eq!(manifest.capabilities, vec!["gmail"]);
}
