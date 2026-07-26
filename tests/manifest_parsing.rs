use epistem::manifest::JsonManifestParser;
use epistem::manifest::ManifestParser;

#[test]
fn parses_manifest() {
    let parser = JsonManifestParser;
    let manifest = parser
        .parse_path(std::path::Path::new("examples/gmail-send/epistem.json"))
        .expect("manifest should parse");

    assert_eq!(manifest.name, "@epistem/gmail-send");
    assert_eq!(manifest.version, "1.0.0");
    assert_eq!(manifest.contracts[0].id, "send-email");
}
