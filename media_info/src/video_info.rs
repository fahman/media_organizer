use ffmpeg_next as ffmpeg;
use std::path::PathBuf;
use log::warn;
use fs_metadata::file_modified;
use crate::counter::increment_fallback_counter;

/// Reads the creation date of the video from the metadata.
///
/// If it can't read the video's creation date, it will fall back to the file's modification date.
/// Function indicates that fallback took place by adding '.fallback' at the end of the date string
/// or by returning 'no.date' in case if no date could be determined.
/// # Examples
/// ```
/// use media_info::read_video_creation_date;
///
/// let video_path = "tests/data/test_video.mp4";
/// let creation_date = read_video_creation_date(video_path).unwrap();
/// assert_eq!(creation_date, "2021-05-21");
/// ```
pub fn read_video_creation_date(path_str: &str) -> Result<String, String> {
    ffmpeg::init().expect("could not initialize ffmpeg");

    match ffmpeg::format::input(&PathBuf::from(path_str)) {
        Ok(context) => {
            let mut creation_date: String = String::new();

            for (name, value) in context.metadata().iter() {
                if name == "creation_time" {
                    creation_date.push_str(value)
                }
            }

            if creation_date.len() == 0 {
                return fallback_to_file_modified_date(path_str);
            }

            if creation_date.split('T').count() > 1 {
                Ok(creation_date.split('T').next().unwrap().to_string())
            } else {
                Ok(creation_date.split_whitespace().next().unwrap().to_string())
            }
        }
        Err(_) => {
            fallback_to_file_modified_date(path_str)
        }
    }
}

fn fallback_to_file_modified_date(path_str: &str) -> Result<String, String> {
    let file_modified_date_string = file_modified(path_str)?;
    let final_file_modified_date = if file_modified_date_string.split('T').count() > 1 {
        match file_modified_date_string.split('T').next() {
            Some(date) => format!("{date}.fallback"),
            None => "no.date".to_string(),
        }
    } else {
        match file_modified_date_string.split_whitespace().next() {
            Some(date) => format!("{date}.fallback"),
            None => "no.date".to_string(),
        }
    };
    increment_fallback_counter();
    warn!("Error reading exif: {:?}, falling back to file modification date {:?}", path_str, final_file_modified_date);
    Ok(final_file_modified_date)
}
