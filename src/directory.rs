use std::{fmt::Error, fs, path::Path};

pub struct EdigeoDir;

pub fn extract_files<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    if path.as_ref().is_dir() {
        let files_paths = path.as_ref().read_dir().unwrap();

        for file in fs::read_dir(path.as_ref())? {
            let entry = file?;
        }
    }

    Ok(())
}
