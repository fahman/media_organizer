use faccess::PathExt;
use std::env;
use std::fs::{copy, rename};
use std::path::{Path, PathBuf};
use log::{debug, error, warn};
use crate::organizer::compare_files::compare_files;
use crate::organizer::counter::{increment_identical_file_counter, increment_same_name_diff_content_counter, increment_saved_file_counter, increment_successfully_compared_file_counter};
#[cfg(target_os = "windows")]
use crate::organizer::set_creation_time_windows::copy_file_metadata;

fn handle_if_removable(file: &str) {
    if !Path::new(file).writable() {
        warn!("{} is not removable. Check file permissions of parent folder?", file)
    }
}

fn media_action(original_file: &str, destination_dir: &str, destination_file_name: &str) {
    let mut dest_file: String = destination_dir.to_owned();
    dest_file.push(std::path::MAIN_SEPARATOR);
    dest_file.push_str(destination_file_name);

    let copy_env = env::var("COPY").expect("COPY not set");
    let dry_run_env = env::var("DRY_RUN").unwrap_or_default();
    let log_saved_env = env::var("LOG_SAVED").unwrap_or_default();

    let destination_path = Path::new(dest_file.as_str());
    if destination_path.exists() {
        match compare_files(original_file, dest_file.as_str()) {
            Ok(false) => {
                dest_file.clear();
                dest_file.push_str(destination_dir);
                dest_file.push(std::path::MAIN_SEPARATOR);
                dest_file.push_str("dup.");
                dest_file.push_str(destination_file_name);

                warn!(target: "same_file", "File {} already exists and it's contents differ from original, saving as {}", destination_file_name, dest_file.as_str());
                increment_same_name_diff_content_counter()
            }
            Ok(true) => {
                debug!(target: "same_file", "File {} already exists and its contents are identical with the original, skipping...", dest_file.as_str());
                increment_identical_file_counter();
                return;
            }
            Err(err) => {
                error!("Comparison of {} and {} has failed, because of {}", original_file, dest_file.as_str(), err);
                return;
            }
        }
    }

    let final_dest = dest_file.as_str();

    if copy_env == "true" {
        if dry_run_env == "false" {
            match copy(original_file, final_dest) {
                Ok(_e) => {
                    increment_saved_file_counter();
                    // if log_saved option was provided, log saved files
                    if log_saved_env == "true" {
                        debug!(target: "saved_file", "Saved {final_dest:?}")
                    }

                    if cfg!(target_os = "windows") {
                        match copy_file_metadata(original_file, final_dest) {
                            Err(err) => error!("Copying of meta data has failed for {final_dest:?}, cause: {err:?}"),
                            Ok(_) => (),
                        }
                    }
                    match compare_files(original_file, final_dest) {
                        Ok(false) => error!("Copy of the file {} has different content", original_file),
                        Ok(true) => increment_successfully_compared_file_counter(),
                        Err(err) => error!("Comparison of {} and {} has failed, because of {}", original_file, final_dest, err),
                    }
                }
                Err(_) => handle_if_removable(original_file),
            };
        } else {
            increment_saved_file_counter();
            increment_successfully_compared_file_counter();
        }
    } else {
        if dry_run_env == "false" {
            match rename(original_file, final_dest) {
                Ok(_e) => {
                    increment_saved_file_counter();
                    // if log_saved option was provided, log saved files
                    if log_saved_env == "true" {
                        debug!(target: "saved_file", "Saved {final_dest:?}")
                    }

                    if cfg!(target_os = "windows") {
                        match copy_file_metadata(original_file, final_dest) {
                            Ok(_) => (),
                            Err(_) => handle_if_removable(original_file),
                        }
                    }
                }
                Err(_) => handle_if_removable(original_file),
            };
        } else {
            increment_saved_file_counter()
        }
    }
}

pub fn handle_media(original_file: &str, dest_dir: &str) {
    let mut original_file_path_buf: PathBuf = PathBuf::new();

    original_file_path_buf.push(original_file);

    match original_file_path_buf.file_name() {
        Some(file_name) => media_action(original_file, dest_dir, file_name.to_str().expect("could not read filename from path buffer.")),
        None => error!("Could not get file name from path: {}", original_file),
    }
}
