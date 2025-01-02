use media_info::{read_audio_creation_date, read_photo_creation_date, read_video_creation_date};
use std::env;

enum DirString<'a> {
    DateBreakdown(Option<&'a str>),
    RegularStr(String),
}

fn finally_make_date_str(appender: String) -> String {
    let dest_folder = env::var("DEST_FOLDER").expect("DEST_FOLDER not set");
    let mut regular_date_folder: String = String::new();

    regular_date_folder.push_str("./");
    regular_date_folder.push_str(&dest_folder);
    regular_date_folder.push('/');
    regular_date_folder.push_str(&appender);

    regular_date_folder
}

fn make_dir_string(date_time: DirString) -> String {
    match date_time {
        DirString::DateBreakdown(breakdown) => {
            if let Some(breakdown) = breakdown {
                let replace_date_hyphens = str::replace(breakdown, "-", "/");
                finally_make_date_str(replace_date_hyphens)
            } else {
                finally_make_date_str(String::from("nodatesexist"))
            }
        }
        DirString::RegularStr(reg_string) => finally_make_date_str(reg_string),
    }
}

pub fn make_photo_dir_str(dir_str: &str) -> String {
    match read_photo_creation_date(dir_str) {
        Ok(date_of_photo) => make_dir_string(DirString::DateBreakdown(
            date_of_photo.split_whitespace().next(),
        )),
        Err(err) => make_dir_string(DirString::RegularStr(err)),
    }
}

pub fn make_video_dir_str(dir_str: &str) -> String {
    match read_video_creation_date(dir_str) {
        Ok(date) => make_dir_string(DirString::DateBreakdown(date.split('T').next())),
        Err(err) => make_dir_string(DirString::RegularStr(err)),
    }
}

pub fn make_audio_dir_str(dir_str: &str) -> String {
    match read_audio_creation_date(dir_str) {
        Ok(date) => make_dir_string(DirString::DateBreakdown(Some(&date))),
        Err(err) => make_dir_string(DirString::RegularStr(err)),
    }
}

#[cfg(test)]
pub mod date_read_tests {
    use super::*;

    #[test]
    fn can_read_photo_creation_date() {
        env::set_var(
            "DEST_FOLDER",
            &format!("tests{}data", std::path::MAIN_SEPARATOR),
        );

        let path_str = &format!(
            ".{}tests{0}data{0}test_photo.JPG",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_photo_creation_date(path_str) {
            Ok(date_of_photo) => make_dir_string(DirString::DateBreakdown(
                date_of_photo.split_whitespace().next(),
            )),
            Err(err) => make_dir_string(DirString::RegularStr(String::from(err))),
        };

        assert_eq!(
            format!("tests{}data{0}2020{0}02{0}01", std::path::MAIN_SEPARATOR),
            date_info
        );
    }

    #[test]
    fn can_read_video_creation_date() {
        env::set_var(
            "DEST_FOLDER",
            &format!("tests{}data", std::path::MAIN_SEPARATOR),
        );

        let path_str = &format!(
            ".{}tests{0}data{0}test_video.mp4",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_video_creation_date(path_str) {
            Ok(date_of_video) => {
                make_dir_string(DirString::DateBreakdown(date_of_video.split("T").next()))
            }
            Err(err) => make_dir_string(DirString::RegularStr(String::from(err))),
        };

        assert_eq!(
            format!("tests{}data{0}2021{0}05{0}21", std::path::MAIN_SEPARATOR),
            date_info
        );
    }
}
