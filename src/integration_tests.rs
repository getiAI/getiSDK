use std::path::PathBuf;
use super::api::simulate_api_call;
use super::markdown::read_markdown_file;
use super::extraction::extract_contracts;

#[tokio::test]
async fn test_api_integration() {
    // Set up the contract files and markdown file paths
    let contract_path = PathBuf::from("test_contracts"); // Replace with the actual path
    let markdown_path = PathBuf::from("markdown.md"); // Replace with the actual path

    // Extract the contracts
    let contracts = extract_contracts(Some(contract_path.to_str().unwrap()));
    
    // Read the markdown content
    let markdown_content = read_markdown_file(markdown_path.to_str().unwrap())
        .expect("Failed to read markdown file");

    // Simulate the API call
    match simulate_api_call(contracts, markdown_content).await {
        Ok(report_link) => {
            println!("Audit completed successfully. Report link: {}", report_link);
            assert!(!report_link.is_empty(), "Report link should not be empty");
        }
        Err(e) => panic!("API call failed: {}", e),
    }
}
