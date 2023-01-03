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
    let mut target_photos = Vec::<&PathBuf>::new();
    for f in camera_dir.photo_filenames.iter() {
        if !target_dir_photos.contains(f) {
            target_photos.push(f);
        }
    }

}
