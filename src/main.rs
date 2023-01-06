use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::camera_dir::CameraDir;
mod camera_dir;


fn main() {
    let DEFAULT_TARGET_PATH: String = "~/Pictures".to_string();
    let DEFAULT_ENABLED_EXT: Vec<String> = vec![
        "jpg".to_string(),
        "JPG".to_string(),
        "jpeg".to_string(),
        "JPEG".to_string(),
        "png".to_string(),
        "PNG".to_string(),
        "mp4".to_string(),
        "MP4".to_string(),
    ];


    let args: Vec<String> = env::args().collect();
    assert!(
        args.len()-1==1 || args.len()-1==2,
        "Expected args are 1 or 2, but given were {}.",
        args.len()-1
    );

    let target_path = if args.len()-1 == 2 {
        &args[2]
    } else {
        &DEFAULT_TARGET_PATH
    };
    let camera_path = &args[1];
    let enabled_ext = &DEFAULT_ENABLED_EXT;

    let camera_dir = CameraDir::new(camera_path, enabled_ext);
    let target_dir = CameraDir::new(target_path, enabled_ext);

    println!("{:?}", camera_dir);
    println!("{:?}", target_dir);

    let target_dir_photos: HashSet<PathBuf> = target_dir.photo_filenames.iter().cloned().collect();
    let mut target_photos = Vec::<(&PathBuf, exif::Exif)>::new();
    for path in camera_dir.photo_filenames.iter() {
        if !target_dir_photos.contains(path) {
            let file = fs::File::open(path).unwrap();
            let mut buf_reader = std::io::BufReader::new(file);
            let exif_reader = exif::Reader::new();
            let exif;
            match exif_reader.read_from_container(&mut buf_reader) {
                Ok(e) => { exif = e; },
                Err(_) => { continue; }
            }

            target_photos.push((path, exif));
        }
    }

    let first_elem = &target_photos[0];
    let res = get_date_path(&target_dir.path, &first_elem.1);
    println!("{:?}", res.unwrap());
}

fn get_date_path(target_dir_pathbuf: &PathBuf, exif: &exif::Exif) -> Option<PathBuf> {
    let datetime_value;
    match exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY) {
        Some(datetime) => { datetime_value = datetime.display_value().to_string(); },
        None => { return None },
    }

    let yymmdd = parse_to_yymmdd(&datetime_value);
    let mut path_string = target_dir_pathbuf.clone().into_os_string().into_string().unwrap();
    path_string += "/";
    path_string += &yymmdd.0;
    path_string += "/";
    path_string += &yymmdd.1;
    path_string += "/";
    path_string += &yymmdd.2;

    return Some(PathBuf::from(path_string));
}

fn parse_to_yymmdd(datetime_value: &String) -> (String, String, String) {
    let mut res = ("".to_string(), "".to_string(), "".to_string());
    res.0 = datetime_value[0..4].to_string();
    res.1 = datetime_value[5..7].to_string();
    res.2 = datetime_value[8..10].to_string();

    return res;
}
