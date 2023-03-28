//! Tests for parsing RustSec advisories

#![warn(rust_2018_idioms, unused_qualifications)]

use rustsec::advisory::Category;
use std::path::Path;

/// Example RustSec Advisory to use for tests
const EXAMPLE_V3_ADVISORY_PATH: &str = "./tests/support/example_advisory_v3.md";

/// Load example V3 advisory from the filesystem
fn load_example_v3_advisory() -> rustsec::Advisory {
    rustsec::Advisory::load_file(Path::new(EXAMPLE_V3_ADVISORY_PATH)).unwrap()
}

/// Load example V4 advisory from the filesystem
fn load_example_v4_advisory() -> rustsec::Advisory {
    rustsec::Advisory::load_file(Path::new("./tests/support/example_advisory_v4.md")).unwrap()
}

/// Basic metadata
#[test]
fn parse_metadata() {
    for advisory in &[load_example_v3_advisory(), load_example_v4_advisory()] {
        assert_eq!(advisory.metadata.id.as_str(), "RUSTSEC-2001-2101");
        assert_eq!(advisory.metadata.package.as_str(), "base");
        assert_eq!(advisory.title(), "All your base are belong to us");
        assert_eq!(
            advisory.description(),
            "You have no chance to survive. Make your time."
        );
        assert_eq!(advisory.metadata.date.as_str(), "2001-02-03");
        assert_eq!(
            advisory.metadata.url.as_ref().unwrap().to_string(),
            "https://www.youtube.com/watch?v=jQE66WA2s-A"
        );

        for (i, category) in [Category::CodeExecution, Category::PrivilegeEscalation]
            .iter()
            .enumerate()
        {
            assert_eq!(*category, advisory.metadata.categories[i]);
        }

        for (i, kw) in ["how", "are", "you", "gentlemen"].iter().enumerate() {
            assert_eq!(*kw, advisory.metadata.keywords[i].as_str());
        }
    }
}

/// Parsing of impact metadata
#[test]
fn parse_affected() {
    let affected = load_example_v3_advisory().affected.unwrap();
    assert_eq!(affected.arch[0], platforms::target::Arch::X86);
    assert_eq!(affected.os[0], platforms::target::OS::Windows);

    let example_function = "base::belongs::All".parse().unwrap();
    let req = &affected.functions.get(&example_function).unwrap()[0];
    assert!(req.matches(&"1.2.2".parse().unwrap()));
    assert!(!req.matches(&"1.2.3".parse().unwrap()));
}

/// Parsing of other aliased advisory IDs
#[test]
fn parse_aliases() {
    let alias = &load_example_v3_advisory().metadata.aliases[0];
    assert!(alias.is_cve());
    assert_eq!(alias.year().unwrap(), 2001);
}

/// Parsing of CVSS v3.1 severity vector strings
#[test]
fn parse_cvss_vector_string() {
    let advisory = load_example_v3_advisory();
    assert_eq!(
        advisory.severity().unwrap(),
        rustsec::advisory::Severity::Critical
    );

    let cvss = advisory.metadata.cvss.unwrap();
    assert_eq!(cvss.av.unwrap(), cvss::v3::base::AttackVector::Network);
    assert_eq!(cvss.ac.unwrap(), cvss::v3::base::AttackComplexity::Low);
    assert_eq!(cvss.pr.unwrap(), cvss::v3::base::PrivilegesRequired::None);
    assert_eq!(cvss.ui.unwrap(), cvss::v3::base::UserInteraction::None);
    assert_eq!(cvss.s.unwrap(), cvss::v3::base::Scope::Changed);
    assert_eq!(cvss.c.unwrap(), cvss::v3::base::Confidentiality::High);
    assert_eq!(cvss.i.unwrap(), cvss::v3::base::Integrity::High);
    assert_eq!(cvss.a.unwrap(), cvss::v3::base::Availability::High);
    assert_eq!(cvss.score().value(), 10.0);
}

/// Parsing of patched version reqs
#[test]
fn parse_patched_version_reqs() {
    let advisory = load_example_v3_advisory();
    let req = &advisory.versions.patched()[0];
    assert!(!req.matches(&"1.2.2".parse().unwrap()));
    assert!(req.matches(&"1.2.3".parse().unwrap()));
    assert!(req.matches(&"1.2.4".parse().unwrap()));
}
