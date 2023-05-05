use std::path::PathBuf;

#[derive(Debug)]
pub struct CameraDir {
    pub path: PathBuf,
    pub photo_filenames: Vec<PathBuf>,
    pub enabled_ext: Vec<String>,
}

impl CameraDir {
    pub fn new(camera_path: &String, enabled_ext: &Vec<String>) -> CameraDir {
        let pathbuf = PathBuf::from(camera_path);

        assert!(
            pathbuf.exists(),
            "Given camera path does not exist."
        );

        println!("check: Given path {:?} exists.", &pathbuf);

        let photo_filenames = Self::search_photos(&pathbuf, enabled_ext);
        return CameraDir { path: pathbuf, photo_filenames: photo_filenames, enabled_ext: enabled_ext.clone() }
    }

    fn search_photos(camera_path: &PathBuf, enabled_ext: &Vec<String>) -> Vec<PathBuf> {
        let mut photo_filenames = Vec::<PathBuf>::new();
        Self::enum_photos(&mut photo_filenames, &camera_path, enabled_ext);

        return photo_filenames;
    }

    fn enum_photos(photo_filenames: &mut Vec<PathBuf>, target_path: &PathBuf, enabled_ext: &Vec<String>) {
        let files;
        match target_path.read_dir() {
            Ok(v) => { files = v; },
            Err(_) => return,
        }

        for dir_entry in files {
            if dir_entry.is_err() {
                continue;
            }

            let path = dir_entry.unwrap().path();
            if path.is_dir() {
                Self::enum_photos(photo_filenames, &path, enabled_ext);
                continue;
            }

            match path.extension() {
                Some(path_ext) => {
                    if enabled_ext.iter().any(|ext| path_ext==ext.as_str()) {
                        photo_filenames.push(path);
                    }
                },
                None => (),
            }
        }
    }
}
