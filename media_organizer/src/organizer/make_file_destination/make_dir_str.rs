use crate::organizer::make_file_destination::MakeFileDestinationError;
use media_info::{read_audio_creation_date, read_photo_creation_date, read_video_creation_date};
use std::env;
use std::path::Path;

fn make_dir_string(date: &str) -> String {
    let replace_date_hyphens = str::replace(date, "-", &std::path::MAIN_SEPARATOR.to_string());
    let dest_folder = env::var("DEST_FOLDER").expect("DEST_FOLDER not set");
    let mut regular_date_folder: String = String::new();

    regular_date_folder.push_str(&dest_folder);
    regular_date_folder.push(std::path::MAIN_SEPARATOR);
    regular_date_folder.push_str(&replace_date_hyphens);

    regular_date_folder
}

pub(crate) fn make_photo_dir_str(date: &str) -> Result<String, MakeFileDestinationError> {
    match read_photo_creation_date(Path::new(date)) {
        Ok(date) => Ok(make_dir_string(date.as_str())),
        Err(error) => Err(MakeFileDestinationError::Error(error)),
    }
}

pub(crate) fn make_video_dir_str(date: &str) -> Result<String, MakeFileDestinationError> {
    match read_video_creation_date(Path::new(date)) {
        Ok(date) => Ok(make_dir_string(date.as_str())),
        Err(error) => Err(MakeFileDestinationError::Error(error)),
    }
}

pub(crate) fn make_audio_dir_str(dir_str: &str) -> Result<String, MakeFileDestinationError> {
    match read_audio_creation_date(Path::new(dir_str)) {
        Ok(date) => Ok(make_dir_string(date.as_str())),
        Err(error) => Err(MakeFileDestinationError::Error(error)),
    }
}

#[cfg(test)]
pub mod date_read_tests {
    use super::*;

    #[test]
    fn can_read_photo_creation_date() {
        env::set_var(
            "DEST_FOLDER",
            &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
        );

        let path_str = &format!(
            "..{}test-media{0}400a861d-014a-4dfb-9143-1a914212fd4d.jpg",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_photo_creation_date(Path::new(path_str)) {
            Ok(date_of_photo) => make_dir_string(date_of_photo.as_str()),
            Err(err) => panic!("Test failed because of error: {}", err),
        };

        assert_eq!(
            format!("tests{}test_files{0}2024{0}10{0}22", std::path::MAIN_SEPARATOR),
            date_info
        );
    }

    #[test]
    fn can_read_video_creation_date() {
        env::set_var(
            "DEST_FOLDER",
            &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
        );

        let path_str = &format!(
            "..{}test-media{0}corgi_race.mp4",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_video_creation_date(Path::new(path_str)) {
            Ok(date_of_video) => make_dir_string(date_of_video.as_str()),
            Err(err) => panic!("Test failed because of error: {}", err),
        };

        assert_eq!(
            format!("tests{}test_files{0}2024{0}10{0}20", std::path::MAIN_SEPARATOR),
            date_info
        );
    }

    #[test]
    fn can_read_audio_creation_date() {
        env::set_var(
            "DEST_FOLDER",
            &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
        );

        let path_str = &format!(
            "..{}test-media{0}Recording.m4a",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_audio_creation_date(Path::new(path_str)) {
            Ok(date_of_audio) => make_dir_string(date_of_audio.as_str()),
            Err(err) => panic!("Test failed because of error: {}", err),
        };

        assert_eq!(
            format!("tests{}test_files{0}2024{0}11{0}11", std::path::MAIN_SEPARATOR),
            date_info
        );
    }
}
