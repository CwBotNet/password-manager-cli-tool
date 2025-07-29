use std::process::Command;

use anyhow::Result;
use clap::{Parser, Subcommand};

use commands::*;
use models::credential::Credential;
use utils::{Write, io, process};

use storage::file::FileStorage;
mod commands;
mod helper;
mod models;
mod storage;
mod utils;

#[derive(Parser)]
#[command(name = "pwdmgr")]
#[command(about = "A secure password manager CLI tool")]
#[commad(version = "1.0.0")]
#[command(author = "Raj sahani \n Gmail: rajsahaniofficial19@gmail.com")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// initialize a new password vault
    Init,

    /// Add a new credential to the vault
    Add {
        /// Title/name for the credential
        title: String,

        /// Username (Optional)
        #[arg(short, long)]
        username: Option<String>,

        /// URL (optional)
        #[arg(short = 'u', long)]
        url: Option<String>,

        /// Notes (optional)
        #[arg(short, long)]
        notes: Option<String>,

        /// Genrate random password
        #[arg(short = 'g', long)]
        generate: bool,
    },

    /// List all credentials
    List {
        /// Show Passwords (hidden by default)
        shoe_password: bool,
    },

    /// Get a Specific Credential
    Get {
        /// Search term (title, username, or ID)
        query: String,

        /// Copy password to clipboard
        #[arg(short, long)]
        copy: bool,
    },

    /// Delete a credential
    Delete {
        /// Id or Search term for Credential to delete
        query: String,

        /// Skip Confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Search credentials by term
    Search {
        /// Search term
        term: String,
    },

    /// Change master Password
    Changepassword,

    /// Show vault statistics
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parser();

    match cli.command {
        Commands::Init => Commands::Init::run(),
        Commands::Add {
            title,
            username,
            url,
            notes,
            generate,
        } => commands::add::run(title, username, url, notes, generate),
        Commands::List { shoe_password } => commands::list::run(shoe_password),
        Commands::Get { query, copy } => commands::get::run(query, copy),
        Commands::Delete { query, force } => commands::delete::run(query, force),
        Commands::Search { term } => commands::search::run(term),
        Commands::Changepassword => commands::change_password::run(),
        Commands::Status => commands::status::run(),
    }
}
