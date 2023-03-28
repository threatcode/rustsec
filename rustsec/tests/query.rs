//! Tests for parsing RustSec advisories

#![warn(rust_2018_idioms, unused_qualifications)]

use rustsec::{advisory::Severity, database::Query, package};

/// Load example advisory from the filesystem
fn load_advisory() -> rustsec::Advisory {
    rustsec::Advisory::load_file("./tests/support/example_advisory_v3.md").unwrap()
}

#[test]
fn matches_name() {
    let advisory = load_advisory();

    let package_matches: package::Name = "base".parse().unwrap();
    let query_matches = Query::new().package_name(package_matches);
    assert!(query_matches.matches(&advisory));

    let package_nomatch: package::Name = "somethingelse".parse().unwrap();
    let query_nomatch = Query::new().package_name(package_nomatch);
    assert!(!query_nomatch.matches(&advisory));
}

#[test]
fn matches_year() {
    let advisory = load_advisory();

    let query_matches = Query::new().year(2001);
    assert!(query_matches.matches(&advisory));

    let query_nomatch = Query::new().year(2525);
    assert!(!query_nomatch.matches(&advisory));
}

#[test]
fn matches_severity() {
    let advisory = load_advisory();

    let query_matches = Query::new().severity(Severity::Critical);
    assert!(query_matches.matches(&advisory));
}
