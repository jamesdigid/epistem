use epistem::manifest::{CapabilityManifest, ContractSpec, DependencySpec};
use epistem::resolver::{DependencyResolver, PetgraphDependencyResolver};

#[test]
fn builds_dependency_graph() {
    let manifests = vec![
        CapabilityManifest {
            name: "@epistem/authenticate-google".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            contracts: vec![ContractSpec {
                id: "authenticate-google".to_string(),
                version: "^1.0".to_string(),
            }],
            dependencies: vec![],
            keywords: vec![],
            artifacts: None,
        },
        CapabilityManifest {
            name: "@epistem/gmail-send".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            contracts: vec![ContractSpec {
                id: "send-email".to_string(),
                version: "^1.0".to_string(),
            }],
            dependencies: vec![DependencySpec {
                contract: "authenticate-google".to_string(),
            }],
            keywords: vec![],
            artifacts: None,
        },
    ];

    let graph = PetgraphDependencyResolver
        .build(&manifests)
        .expect("graph should build");

    assert_eq!(
        graph.nodes(),
        vec![
            "@epistem/authenticate-google".to_string(),
            "@epistem/gmail-send".to_string()
        ]
    );
    assert_eq!(graph.unresolved_requirements(), Vec::<String>::new());
    assert_eq!(graph.topological_order().expect("toposort"), graph.nodes());
    assert_eq!(graph.edges().len(), 1);
    assert_eq!(graph.edges()[0].contract, "authenticate-google");
}
