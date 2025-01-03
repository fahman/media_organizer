use exif::{In, Reader, Tag as ExifTag};
use fs_metadata::file_modified;
use std::fs::File;
use std::path::Path;
use log::warn;
use crate::counter::increment_fallback_counter;

/// Reads the creation date of the photo from the metadata.
///
/// If it can't read the photo's creation date, it will fall back to the file's modification date.
/// Function indicates that fallback took place by adding '.fallback' at the end of the date string
/// or by returning 'no.date' in case if no date could be determined.
///
/// # Examples
/// ```
/// use media_info::read_photo_creation_date;
///
/// let photo_path = "./tests/data/test_photo.JPG";
/// let creation_date = read_photo_creation_date(photo_path).unwrap();
/// assert_eq!(creation_date, "2020-02-01");
/// ```
pub fn read_photo_creation_date(path_str: &str) -> Result<String, String> {
    let file = File::open(Path::new(path_str)).expect("could not open photo");
    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = Reader::new();

    match exif_reader.read_from_container(&mut bufreader) {
        Ok(reader) => match reader.get_field(ExifTag::DateTimeOriginal, In::PRIMARY) {
            Some(data) => Ok(data.value.display_as(data.tag).to_string().split_whitespace().next().unwrap().to_string()),
            None => fallback_to_file_modified_date(path_str)
        }
        Err(_) => fallback_to_file_modified_date(path_str)
    }
}

fn fallback_to_file_modified_date(path_str: &str) -> Result<String, String> {
    let file_modified_date_string = file_modified(path_str)?;
    // indicate that given date is a fallback or that there is no date at all
    let final_file_modified_date = match file_modified_date_string.split_whitespace().next() {
        Some(date) => format!("{date}.fallback"),
        None => "no.date".to_string(),
    };
    increment_fallback_counter();
    warn!("Error reading exif: {:?}, falling back to file modification date {:?}", path_str, final_file_modified_date);
    Ok(final_file_modified_date.to_string())
}
