use std::env;
use std::fs;

use crate::camera_dir::CameraDir;
mod camera_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len()-1==1 || args.len()-1==2,
        "Expected args are 1 or 2, but given were {}.",
        args.len()-1
    );

    let target_path = if args.len()-1 == 2 {
        &args[2]
    } else {
        "~/Pictures"
    };
    let camera_path = &args[1];
    let enabled_ext_str = vec!["jpg", "JPG", "jpeg", "png"];
    let mut enabled_ext = Vec::<String>::new();
    for e in enabled_ext_str.iter() {
        enabled_ext.push(e.to_string());
    }

    let camera_dir = CameraDir::new(camera_path, &enabled_ext);

    println!("{:?}", camera_dir);
}
