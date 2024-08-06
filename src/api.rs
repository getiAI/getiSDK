// // src/api.rs

// use std::path::PathBuf;

// pub fn simulate_api_call(contracts: Vec<PathBuf>, markdown_content: String) -> Result<String, String> {
//     // Simulate sending the data to an imaginary API
//     // For now, we'll just return a success message with the count of files and length of markdown content
    
//     if contracts.is_empty() {
//         return Err("No contract files provided".to_string());
//     }

//     if markdown_content.is_empty() {
//         return Err("Markdown content is empty".to_string());
//     }

//     Ok(format!(
//         "Successfully sent {} contract files and a markdown file with {} characters to the API.",
//         contracts.len(),
//         markdown_content.len()
//     ))
// }


use std::path::PathBuf;
use reqwest::multipart;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn simulate_api_call(contracts: Vec<PathBuf>, markdown_content: String) -> Result<String, String> {
    if contracts.is_empty() {
        return Err("No contract files provided".to_string());
    }

    if markdown_content.is_empty() {
        return Err("Markdown content is empty".to_string());
    }

    let mut form = multipart::Form::new()
        .text("markdown", markdown_content);

    let client = reqwest::Client::new();
    
    // Add files to form
    for contract in contracts {
        let mut file = File::open(&contract).await.map_err(|e| format!("Failed to open contract file: {}", e))?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await.map_err(|e| format!("Failed to read contract file: {}", e))?;
        let part = multipart::Part::bytes(contents).file_name(contract.file_name().unwrap().to_str().unwrap().to_string());
        form = form.part("contracts", part);
    }

    let response = client.post("http://localhost:3000/api/audit")
        .multipart(form)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let text = resp.text().await.unwrap();
            Ok(format!("Successfully sent contracts and markdown content. Response: {}", text))
        },
        Err(err) => Err(format!("Failed to send contracts and markdown content. Error: {}", err)),
    }
}

