use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

pub fn file_created(path_str: &Path) -> Result<String, String> {
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

pub fn file_modified(path_str: &Path) -> Result<String, String> {
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

pub fn last_accessed(path_str: &Path) -> Result<String, String> {
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
        let path = Path::new(&format!("..{}test-media{0}400a861d-014a-4dfb-9143-1a914212fd4d.jpg", std::path::MAIN_SEPARATOR));
        let result = file_created(path).unwrap();
        assert_eq!(result, "2024-11-10");
    }

    #[test]
    fn can_read_modified_string() {
        let path = Path::new(&format!("..{}test-media{0}400a861d-014a-4dfb-9143-1a914212fd4d.jpg", std::path::MAIN_SEPARATOR));
        let result = file_modified(path).unwrap();
        assert_eq!(result, "2024-11-10");
    }

    #[test]
    fn can_read_accessed_string() {
        let path = Path::new(&format!("..{}test-media{0}400a861d-014a-4dfb-9143-1a914212fd4d.jpg", std::path::MAIN_SEPARATOR));
        let result = last_accessed(path).unwrap();
        assert_eq!(result, "2024-11-18");
    }
}
