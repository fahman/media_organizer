use chrono::NaiveDate;
use file_metadata::file_created;
use id3::{ErrorKind, Timestamp};
use id3::{Tag, TagLike};
use std::str::FromStr;

fn make_date_recorded_from_file(path_str: &str) -> Option<Timestamp> {
    let formatted_date = file_created(path_str);
    let id3_timestamp = Timestamp::from_str(&formatted_date).expect("could not write timestamp");

    let mut tag = Tag::new();
    tag.set_date_recorded(id3_timestamp);
    tag.write_to_path(path_str, id3::Version::Id3v24)
        .expect("could not write tag");

    return Some(id3_timestamp);
}

pub fn read_audio_creation_date(path_str: &str) -> Result<String, String> {
    let date_recorded = match Tag::read_from_path(path_str) {
        Ok(tags) => tags.date_recorded(),
        Err(why) => match why.kind {
            ErrorKind::NoTag => make_date_recorded_from_file(&path_str),
            _ => None,
        },
    };
    let year = date_recorded.map(|t| t.year).expect("No year found");
    let month = date_recorded
        .map(|t| t.month.unwrap())
        .expect("No month found");
    let day = date_recorded.map(|t| t.day.unwrap()).expect("No day found");
    let assembled_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32);
    let date_str = assembled_date.unwrap().format("%Y-%m-%d").to_string();

    return Ok(date_str);
}