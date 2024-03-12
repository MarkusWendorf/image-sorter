use std::{
    fs, io,
    path::{Path, PathBuf},
};

use chrono::DateTime;
use exif::{Exif, In, Tag};
use itertools::Itertools;

struct ImageFile {
    name: String,
    path: PathBuf,
    date_time: String,
}

fn main() {
    let source_dirs = vec![
        Path::new("/Users/markuswendorf/Documents/santo antao/output/JPEG"),
        Path::new("/Users/markuswendorf/Documents/santo antao/handy selection"),
    ];

    let dest_dir = "/Users/markuswendorf/Documents/santo antao/test-out";

    let images = source_dirs
        .iter()
        .flat_map(|dir| fs::read_dir(dir).unwrap())
        .filter_map(|entry| {
            let file = entry.unwrap();
            let path = &file.path();

            if path.extension().is_none() {
                return None;
            }

            let image = ImageFile {
                name: file.file_name().into_string().unwrap(),
                date_time: get_date_time(path),
                path: file.path(),
            };

            Some(image)
        })
        .sorted_by_key(|image| image.date_time.clone());

    for image in images {
        let new_file = format!("{}/{}_{}", dest_dir, image.date_time, image.name);
        fs::copy(image.path, Path::new(&new_file)).unwrap();
    }
}

fn get_date_time(file_path: &Path) -> String {
    let file = fs::File::open(file_path).unwrap();
    let mut reader = io::BufReader::new(&file);

    let exif = exif::Reader::new()
        .read_from_container(&mut reader)
        .unwrap();

    let exif_date_time = get_exif_field(&exif, Tag::DateTimeDigitized);
    let exif_time_offset = get_exif_field(&exif, Tag::OffsetTimeOriginal);

    let offset = parse_offset(&exif_time_offset);
    parse_date_time(&exif_date_time, offset)
}

fn parse_offset(offset_str: &str) -> &str {
    offset_str.trim_matches(|c| c == '"')
}

fn parse_date_time(date_time_str: &str, offset: &str) -> String {
    DateTime::parse_from_str(&format!("{} {}", date_time_str, offset), "%F %T %:z")
        .unwrap()
        .to_utc()
        .format("%Y%m%d%H%M%S")
        .to_string()
}

fn get_exif_field(exif: &Exif, tag: Tag) -> String {
    exif.get_field(tag, In::PRIMARY)
        .unwrap()
        .value
        .display_as(tag)
        .to_string()
}
