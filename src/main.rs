#![feature(let_chains)]

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use chrono::DateTime;
use clap::Parser;
use exif::{Exif, In, Tag};
use itertools::Itertools;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    out: String,

    #[arg(long, value_parser = clap::value_parser!(PathBuf), num_args = 1..)]
    folders: Vec<PathBuf>,
}

struct ImageFile {
    name: String,
    path: PathBuf,
    date_time: String,
}

fn main() {
    let args = Args::parse();

    let source_dirs = args.folders;
    let dest_dir = args.out;

    let data_dir = Path::new(&dest_dir);
    fs::create_dir_all(data_dir).expect("failed to create output directory");

    let default_timezone_offset = "+02:00".to_owned(); // todo: cli arg

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
                date_time: get_date_time(path, default_timezone_offset.clone()),
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

fn get_date_time(file_path: &Path, default_timezone_offset: String) -> String {
    let file = fs::File::open(file_path).unwrap();
    let mut reader = io::BufReader::new(&file);

    let exif = exif::Reader::new()
        .read_from_container(&mut reader)
        .unwrap();

    let date_time = get_next_best_exif_field(
        &exif,
        &[Tag::DateTimeDigitized, Tag::DateTimeOriginal, Tag::DateTime],
    );

    let time_offset = get_next_best_exif_field(
        &exif,
        &[
            Tag::OffsetTimeDigitized,
            Tag::OffsetTimeOriginal,
            Tag::OffsetTime,
        ],
    );

    parse_date_time(
        &date_time.expect("no date time metadata found"),
        parse_offset(&time_offset.unwrap_or(default_timezone_offset)),
    )
}

fn parse_offset(offset_str: &str) -> &str {
    offset_str.trim_matches(|c| c == '"')
}

fn parse_date_time(date_time_str: &str, offset: &str) -> String {
    println!("{}, {}", date_time_str, offset);
    DateTime::parse_from_str(&format!("{} {}", date_time_str, offset), "%F %T %:z")
        .unwrap()
        .to_utc()
        .format("%Y%m%d%H%M%S")
        .to_string()
}

fn get_exif_field(exif: &Exif, tag: Tag) -> Option<String> {
    exif.get_field(tag, In::PRIMARY)
        .map(|field| field.value.display_as(tag).to_string())
}

fn get_next_best_exif_field(exif: &Exif, tags: &[Tag]) -> Option<String> {
    for &tag in tags {
        if let Some(field) = get_exif_field(exif, tag) {
            return Some(field);
        }
    }

    return None;
}
