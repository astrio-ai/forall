#![allow(clippy::expect_used)]

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use forall_authoring::authoring::{
    CanonicalRoot, ContractTarget, MutationMode, MutationOptions, ScaffoldContractsRequest,
    SourceLanguage, UpsertRequirementsRequest, discover_project_symbols, init_project,
    project_status, scaffold_contracts, upsert_requirements, validate_authoring,
};
use forall_authoring::mapping::schema::{CodeRef, Requirement};
use sha2::{Digest, Sha256};

struct Fixture {
    language: SourceLanguage,
    path: &'static str,
    source: &'static str,
    selector: &'static str,
    requires: &'static str,
    ensures: &'static str,
}

const FIXTURES: &[Fixture] = &[
    Fixture {
        language: SourceLanguage::TypeScript,
        path: "src/clamp.ts",
        source: include_str!("fixtures/typescript/src/clamp.ts"),
        selector: "clamp",
        requires: "lo <= hi",
        ensures: "lo <= result && result <= hi",
    },
    Fixture {
        language: SourceLanguage::Rust,
        path: "src/lib.rs",
        source: include_str!("fixtures/rust/src/lib.rs"),
        selector: "clamp",
        requires: "lo <= hi",
        ensures: "lo <= result && result <= hi",
    },
    Fixture {
        language: SourceLanguage::Java,
        path: "src/Clamp.java",
        source: include_str!("fixtures/java/src/Clamp.java"),
        selector: "clamp(int x, int lo, int hi)",
        requires: "lo <= hi;",
        ensures: r"lo <= \result && \result <= hi;",
    },
];

fn mutation(expected: impl IntoIterator<Item = (String, String)>) -> MutationOptions {
    MutationOptions {
        mode: MutationMode::Apply,
        expected_sha256: expected.into_iter().collect(),
    }
}

fn hash(path: &Path) -> String {
    format!(
        "{:x}",
        Sha256::digest(fs::read(path).expect("fixture file"))
    )
}

#[test]
fn authors_and_validates_three_brownfield_languages() {
    for fixture in FIXTURES {
        let directory = tempfile::tempdir().expect("tempdir");
        let source_path = directory.path().join(fixture.path);
        fs::create_dir_all(source_path.parent().expect("source parent")).expect("source directory");
        fs::write(&source_path, fixture.source).expect("source fixture");
        let root = CanonicalRoot::new(directory.path()).expect("canonical root");

        init_project(&root, &mutation(BTreeMap::new())).expect("initialize");
        let discovered =
            discover_project_symbols(&root, &[fixture.language]).expect("discover symbols");
        assert!(
            discovered
                .symbols
                .iter()
                .any(|symbol| symbol.symbol == fixture.selector)
        );

        let status = project_status(&root).expect("project status");
        upsert_requirements(
            &root,
            &UpsertRequirementsRequest {
                requirements: vec![Requirement {
                    id: "clamp-bounds".to_string(),
                    capability: "math".to_string(),
                    requirement: "clamp returns a value within the requested bounds".to_string(),
                    verified: true,
                    property_tested: false,
                    property: None,
                    code: Some(CodeRef {
                        file: fixture.path.to_string(),
                        symbols: vec![fixture.selector.to_string()],
                    }),
                    contract: Some("requires bounds; ensures bounded result".to_string()),
                    claimcheck: None,
                    scenarios: None,
                }],
                mutation: mutation([(
                    ".forall/verify/mapping.yaml".to_string(),
                    status.mapping_sha256.expect("mapping hash"),
                )]),
            },
        )
        .expect("upsert requirement");

        scaffold_contracts(
            &root,
            &ScaffoldContractsRequest {
                contracts: vec![ContractTarget {
                    file: fixture.path.to_string(),
                    symbol: fixture.selector.to_string(),
                    requires: vec![fixture.requires.to_string()],
                    ensures: vec![fixture.ensures.to_string()],
                }],
                mutation: mutation([(fixture.path.to_string(), hash(&source_path))]),
            },
        )
        .expect("scaffold contract");

        let validation = validate_authoring(&root).expect("validate authoring");
        assert!(validation.valid, "{:?}", validation.issues);
    }
}
