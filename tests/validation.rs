use epistem::manifest::ManifestValidator;

#[test]
fn validates_manifest() {
    let validator = ManifestValidator::default();
    let report = validator.validate_path("examples/gmail-send");

    assert!(report.valid);
    let manifest = report.manifest.expect("manifest should be present");
    assert_eq!(manifest.name, "@epistem/gmail-send");
}
