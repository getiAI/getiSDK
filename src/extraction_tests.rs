// src/extraction_tests.rs

#[cfg(test)]
mod tests {
    use super::super::{init, extraction::extract_contracts};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_extract_contracts_current_dir() {
        init();
        let contract_files = extract_contracts(None);
        assert!(!contract_files.is_empty());
    }

    #[test]
    fn test_extract_contracts_specified_dir() {
        init();
        let test_dir = "test_contracts";
        fs::create_dir_all(test_dir).unwrap();
        fs::write(Path::new(test_dir).join("test1.rs"), "contract code").unwrap();
        fs::write(Path::new(test_dir).join("test2.txt"), "not a contract").unwrap();

        let contract_files = extract_contracts(Some(test_dir));
        assert_eq!(contract_files.len(), 1);
        assert_eq!(contract_files[0].file_name().unwrap(), "test1.rs");

        fs::remove_dir_all(test_dir).unwrap();
    }
}
