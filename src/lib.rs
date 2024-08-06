// // src/lib.rs

// pub mod extraction;
// pub mod markdown;
// pub mod cli;
// pub mod api;
// mod extraction_tests;

// use cli::{Cli, Commands};
// use clap::Parser;
// use log::info;
// use std::sync::Once;

// static INIT: Once = Once::new();

// pub fn init_logging() {
//     INIT.call_once(|| {
//         env_logger::Builder::from_default_env()
//             .filter_level(log::LevelFilter::Info)
//             .init();
//     });
// }

// pub fn init() {
//     init_logging();
// }

// pub fn run() {
//     init();

//     let cli = Cli::parse();

//     match &cli.command {
//         Commands::Extract { path } => {
//             let contract_path = path.as_deref();
//             let files = extraction::extract_contracts(contract_path.map(|p| p.to_str().unwrap()));
//             info!("Extracted files: {:?}", files);
//         }
//         Commands::ReadMarkdown { path } => {
//             match markdown::read_markdown_file(path.to_str().unwrap()) {
//                 Ok(content) => info!("Markdown content: {}", content),
//                 Err(e) => eprintln!("Error: {}", e),
//             }
//         }
//         Commands::Audit {
//             contract_path,
//             markdown_path,
//         } => {
//             let contracts = extraction::extract_contracts(contract_path.as_deref().map(|p| p.to_str().unwrap()));
//             let markdown = match markdown::read_markdown_file(markdown_path.to_str().unwrap()) {
//                 Ok(content) => content,
//                 Err(e) => {
//                     eprintln!("Error: {}", e);
//                     return;
//                 }
//             };

//             match api::simulate_api_call(contracts, markdown) {
//                 Ok(response) => info!("{}", response),
//                 Err(e) => eprintln!("API Error: {}", e),
//             }
//         }
//     }
// }


pub mod extraction;
pub mod markdown;
pub mod cli;
pub mod api;
mod extraction_tests;

use cli::{Cli, Commands};
use clap::Parser;
use log::info;
use std::sync::Once;
use tokio::runtime::Runtime;

static INIT: Once = Once::new();

pub fn init_logging() {
    INIT.call_once(|| {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    });
}

pub fn init() {
    init_logging();
}

pub fn run() {
    init();

    let cli = Cli::parse();

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match &cli.command {
            Commands::Extract { path } => {
                let contract_path = path.as_deref();
                let files = extraction::extract_contracts(contract_path.map(|p| p.to_str().unwrap()));
                info!("Extracted files: {:?}", files);
            }
            Commands::ReadMarkdown { path } => {
                match markdown::read_markdown_file(path.to_str().unwrap()) {
                    Ok(content) => info!("Markdown content: {}", content),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Commands::Audit {
                contract_path,
                markdown_path,
            } => {
                let contracts = extraction::extract_contracts(contract_path.as_deref().map(|p| p.to_str().unwrap()));
                let markdown = match markdown::read_markdown_file(markdown_path.to_str().unwrap()) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return;
                    }
                };

                match api::simulate_api_call(contracts, markdown).await {
                    Ok(response) => info!("{}", response),
                    Err(e) => eprintln!("API Error: {}", e),
                }
            }
        }
    });
}
