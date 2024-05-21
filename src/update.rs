use std::{error::Error, ffi::OsString, fs, io, path::{Path, PathBuf}};

use crate::{OUT_DIR, SRC_DIR};

pub fn run(path: PathBuf) -> Result<usize, Box<dyn Error>> {
    let src_dir = Path::new(&path).join(SRC_DIR);
    let out_dir = Path::new(&path).join(OUT_DIR);

    let out_files: Vec<PathBuf> = fs::read_dir(out_dir)?
        .map(|res| res.map(|file| file.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let src_files: Vec<OsString> = fs::read_dir(&src_dir)?
        .map(|res| res.map(|file| {
            file.file_name()
        }))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut count = 0;
    for out_file in out_files {
        if !src_files.contains(&out_file.file_name().unwrap().to_os_string()) {
            count += 1;

            let src_file = Path::new(&src_dir).join(out_file.file_name().expect("File name of script could not be read."));
            fs::copy(&out_file, &src_file)?;

            println!("Added new script {:?}", out_file.file_name().unwrap());
        }
    }

    Ok(count)
}