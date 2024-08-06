// // src/cli.rs

// use clap::{Parser, Subcommand};
// use std::path::PathBuf;

// #[derive(Parser)]
// #[command(name = "geti")]
// #[command(about = "A tool for auditing Arbitrum stylus Rust smart contracts")]
// pub struct Cli {
//     #[command(subcommand)]
//     pub command: Commands,
// }

// #[derive(Subcommand)]
// pub enum Commands {
//     /// Extract smart contract files
//     Extract {
//         /// Path to the directory containing the smart contracts
//         #[arg(short, long)]
//         path: Option<PathBuf>,
//     },
//     /// Read and display a Markdown file
//     ReadMarkdown {
//         /// Path to the Markdown file
//         #[arg(short, long)]
//         path: PathBuf,
//     },
//     /// Perform audit (placeholder)
//     Audit {
//         /// Path to the directory containing the smart contracts
//         #[arg(short, long)]
//         contract_path: Option<PathBuf>,
//         /// Path to the Markdown file explaining the project
//         #[arg(short, long)]
//         markdown_path: PathBuf,
//     },
// }


use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "geti")]
#[clap(about = "A tool for auditing Arbitrum stylus Rust smart contracts")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Extract smart contract files
    Extract {
        /// Path to the directory containing the smart contracts
        #[clap(short, long)]
        path: Option<PathBuf>,
    },
    /// Read and display a Markdown file
    ReadMarkdown {
        /// Path to the Markdown file
        #[clap(short, long)]
        path: PathBuf,
    },
    /// Perform audit (placeholder)
    Audit {
        /// Path to the directory containing the smart contracts
        #[clap(short, long)]
        contract_path: Option<PathBuf>,
        /// Path to the Markdown file explaining the project
        #[clap(short, long)]
        markdown_path: PathBuf,
    },
}
