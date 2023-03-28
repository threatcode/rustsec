//! `rustsec-admin` CLI subcommands

mod assign_id;
mod lint;
mod list_affected_versions;
mod osv;
mod version;
mod web;

use self::{
    assign_id::AssignIdCmd, lint::LintCmd, list_affected_versions::ListAffectedVersionsCmd,
    osv::OsvCmd, version::VersionCmd, web::WebCmd,
};
use crate::config::AppConfig;
use abscissa_core::{Command, Configurable, Runnable};
use clap::Parser;
use std::path::PathBuf;

/// `rustsec-admin` CLI subcommands
#[derive(Command, Debug, Parser, Runnable)]
pub enum AdminSubCmd {
    /// The `lint` subcommand
    #[clap(about = "lint Advisory DB and ensure is well-formed")]
    Lint(LintCmd),

    /// The `web` subcommand
    #[clap(about = "render advisory Markdown files for the rustsec.org web site")]
    Web(WebCmd),

    /// The `version` subcommand
    #[clap(about = "display version information")]
    Version(VersionCmd),

    /// The `assign-id` subcommand
    #[clap(about = "assigning RUSTSEC ids to new vulnerabilities")]
    AssignId(AssignIdCmd),

    /// The `osv` subcommand
    #[clap(about = "export advisories to OSV format")]
    Osv(OsvCmd),

    /// The `version` subcommand
    #[clap(about = "list affected crate versions")]
    ListAffectedVersions(ListAffectedVersionsCmd),
}

/// `rustsec-admin` CLI commands
#[derive(Command, Debug, Parser)]
#[clap(author, version, about)]
pub struct AdminCmd {
    #[clap(subcommand)]
    cmd: AdminSubCmd,

    /// Increase verbosity setting
    #[clap(short = 'v', long, help = "Increase verbosity")]
    pub verbose: bool,
}

impl Runnable for AdminCmd {
    fn run(&self) {
        self.cmd.run()
    }
}

impl Configurable<AppConfig> for AdminCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
