mod video_info;
mod audio_info;
mod photo_info;
mod counter;

pub use audio_info::read_audio_creation_date;
pub use photo_info::read_photo_creation_date;
pub use video_info::read_video_creation_date;
pub use counter::get_fallback_counter;