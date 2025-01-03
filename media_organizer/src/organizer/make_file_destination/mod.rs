mod make_dir_str;

use crate::organizer::make_file_destination::make_dir_str::{make_audio_dir_str, make_photo_dir_str, make_video_dir_str};
use std::path::Path;

fn get_white_list_video_types<'a>() -> Vec<&'a str> {
    vec!["mp4", "MP4", "mov", "MOV", "avi", "AVI"]
}

fn get_white_list_photo_types<'a>() -> Vec<&'a str> {
    vec![
        "tiff", "TIFF", "heif", "HEIF", "HEIC", "heic", "AVIF", "avif", "jpeg", "jpg", "JPEG",
        "JPG", "HEIC", "heic", "PNG", "png", "webp", "WEBP", "dng", "DNG", "gif", "GIF", "raw", "RAW",
    ]
}

fn get_white_list_audio_types<'a>() -> Vec<&'a str> {
    vec!["mp3", "MP3", "wav", "WAV", "aiff", "AIFF", "m4a", "M4A", "flac", "FLAC"]
}

fn ends_with_type(types: Vec<&str>, name: &str) -> bool {
    for file_type in types {
        if name.ends_with(file_type) {
            return true;
        }
    }

    false
}

fn is_video(file_name: &str) -> bool {
    ends_with_type(get_white_list_video_types(), file_name)
}

fn is_photo(file_name: &str) -> bool {
    ends_with_type(get_white_list_photo_types(), file_name)
}

fn is_audio(file_name: &str) -> bool {
    ends_with_type(get_white_list_audio_types(), file_name)
}

#[derive(Debug)]
pub enum MakeFileDestinationError {
    UnsupportedType(String),
    Error(String),
}

pub fn make_file_destination_str(file_name: &str) -> Result<String, MakeFileDestinationError> {
    if is_video(file_name) {
        return make_video_dir_str(file_name);
    }
    if is_photo(file_name) {
        return make_photo_dir_str(file_name);
    }
    if is_audio(file_name) {
        return make_audio_dir_str(file_name);
    }

    let path = Path::new(file_name);
    Err(MakeFileDestinationError::UnsupportedType(path.extension().unwrap().to_str().unwrap_or("").to_string()))
}
