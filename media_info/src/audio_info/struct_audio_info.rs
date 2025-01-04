use chrono::NaiveDate;
use id3::{Tag as ID3Tag, TagLike, Timestamp};
use std::path::Path;

#[derive(Debug)]
pub struct AudioInfo {
    pub creation_date: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub duration: String,
    pub released: String,
    pub genre: String,
}

impl AudioInfo {
    pub fn new(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!("File does not exist: {:?}", path));
        }

        let tag = ID3Tag::read_from_path(path).unwrap();

        let date_recorded = get_date_recorded(&tag);
        let artist = get_artist(&tag);
        let title = get_title(&tag);
        let album = get_album(&tag);
        let duration = get_duration(&tag);
        let released = get_release_date(&tag);
        let genre = get_genre(&tag);

        Ok(AudioInfo {
            creation_date: date_recorded,
            artist,
            title,
            album,
            duration,
            released,
            genre,
        })
    }
}

fn format_date(date: Timestamp) -> String {
    let year = date.year;
    let month = date.month.unwrap_or(1);
    let day = date.day.unwrap_or(1);

    let assembled_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32);

    assembled_date.unwrap().format("%Y-%m-%d").to_string()
}

pub fn get_date_recorded(tag: &ID3Tag) -> String {
    let date_recorded = tag.date_recorded().unwrap();
    format_date(date_recorded)
}

pub fn get_artist(tag: &ID3Tag) -> String {
    tag.artist().unwrap_or("Unknown Artist").to_string()
}

pub fn get_title(tag: &ID3Tag) -> String {
    tag.title().unwrap_or("Unknown Title").to_string()
}

pub fn get_album(tag: &ID3Tag) -> String {
    tag.album().unwrap_or("Unknown Album").to_string()
}

pub fn get_duration(tag: &ID3Tag) -> String {
    let duration = tag.duration().unwrap_or(0);
    let duration_str = format!("{} seconds", duration);

    duration_str
}

pub fn get_release_date(tag: &ID3Tag) -> String {
    let release_date = tag.date_released().unwrap_or_default();

    format_date(release_date)
}

pub fn get_genre(tag: &ID3Tag) -> String {
    tag.genre().unwrap_or("Unknown Genre").to_string()
}
