use chrono::{DateTime, Local};
use std::fs;

pub fn file_created(path_str: &str) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(created) = data.created() {
                let datetime: DateTime<Local> = created.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                Ok(formatted_date)
            } else {
                Err("Failed to read file creation date".to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn file_modified(path_str: &str) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(modified) = data.modified() {
                let datetime: DateTime<Local> = modified.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                Ok(formatted_date)
            } else {
                Err("Failed to read file modified date".to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn last_accessed(path_str: &str) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(accessed) = data.accessed() {
                let datetime: DateTime<Local> = accessed.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                Ok(formatted_date)
            } else {
                Err("Failed to read file last accessed date".to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_creation_string() {
        let result = file_created(&format!(".{}tests{0}data{0}test_photo.JPG", std::path::MAIN_SEPARATOR)).unwrap();
        assert_eq!(result, "2025-01-02");
    }

    #[test]
    fn can_read_modified_string() {
        let result = file_modified(&format!(".{}tests{0}data{0}test_photo.JPG", std::path::MAIN_SEPARATOR)).unwrap();
        assert_eq!(result, "2025-01-02");
    }

    #[test]
    fn can_read_accessed_string() {
        let result = last_accessed(&format!(".{}tests{0}data{0}test_photo.JPG", std::path::MAIN_SEPARATOR)).unwrap();
        assert_eq!(result, "2025-01-02");
    }
}
