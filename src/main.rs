use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{ BufReader, Read, Seek, SeekFrom };
use std::path::PathBuf;

use crate::camera_dir::CameraDir;
mod camera_dir;
mod raw;


fn main() {
    let home_dir;
    match env::var("HOME") {
        Ok(v) => { home_dir = v; },
        Err(_) => {
            println!("Could not read $HOME.");
            return
        }
    }
    let default_taregt_path = home_dir + "/Pictures";
    let default_enabled_ext_str = vec![
        "jpg",
        "JPG",
        "jpeg",
        "JPEG",
        "png",
        "PNG",
        "mp4",
        "MP4",
        "CR2",
        "CR3",
        "CRW",
        "RAF",
        "RWL",
        "DNG",
        "NEF",
        "NRW",
        "ORF",
        "RW2",
        "PEF",
        "X3F",
        "ARW",
        "SR2",
        "SRF",
    ];

    let default_enabled_ext = default_enabled_ext_str.iter().map(|&str| str.to_string()).collect();


    let args: Vec<String> = env::args().collect();
    assert!(
        args.len()-1==1 || args.len()-1==2,
        "Expected args are 1 or 2, but given were {}.",
        args.len()-1
    );

    let target_path = if args.len()-1 == 2 {
        &args[2]
    } else {
        &default_taregt_path
    };
    let camera_path = &args[1];
    let enabled_ext = &default_enabled_ext;

    let camera_dir = CameraDir::new(camera_path, enabled_ext);
    let target_dir = CameraDir::new(target_path, enabled_ext);

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
                Err(_) => {
                    match raw::find_tiff_marker(&mut buf_reader) {
                        Ok(v) => {  buf_reader.seek(SeekFrom::Start(v)); },
                        Err(_) => { continue; },
                    }
                    match exif_reader.read_from_container(&mut buf_reader) {
                        Ok(e) => { exif = e; },
                        Err(_) => { continue; },
                    }
                }
            }

            target_photos.push((path, exif));
        }
    }

    let mut copied_file_num = 0;
    for (origin_path, exif) in target_photos {
        let target_photo_dir = get_date_path(&target_dir.path, &exif).unwrap();
        let mut new_path = target_photo_dir.clone();
        new_path.push(origin_path.file_name().unwrap());
        if new_path.is_file() {
            continue;
        }

        print!("{:?} ---> {:?} | moving......", origin_path, new_path);
        match fs::create_dir_all(&target_photo_dir) {
            Ok(_) => (),
            Err(_) => {
                println!("   canceled.");
                continue;
            }
        }

        match fs::copy(origin_path, new_path) {
            Ok(_) => {
                println!("   done!");
                copied_file_num += 1;
            },
            Err(_) => println!("   canceled.")
        }
    }

    println!("{} files has copied!", copied_file_num);
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
