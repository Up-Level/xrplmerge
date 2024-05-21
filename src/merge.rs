use std::{error::Error, fs, path::{Path, PathBuf}, time::{Duration, Instant}};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{LIB_DIR, OUT_DIR, SRC_DIR};

static INCLUDE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^#include \w+\.\w+(\n|\r\n)").unwrap());

pub fn run(path: PathBuf) -> Result<usize, Box<dyn Error>> {
    let mut total_time = Duration::new(0, 0);
    let mut file_count = 0;

    let src_dir = Path::new(&path).join(SRC_DIR);
    let lib_dir = Path::new(&path).join(LIB_DIR);
    let out_dir = Path::new(&path).join(OUT_DIR);

    for file in fs::read_dir(src_dir)? {
        let file = file?;

        if file.file_type()?.is_dir() {continue}

        let source_path = file.path();
        if source_path.extension().unwrap() != "4rpl" {continue}

        let out_path = Path::new(&out_dir).join(source_path.file_name().ok_or("_.4rpl")?);

        let time_taken = merge_src_file(&source_path, &out_path, &lib_dir)?;

        total_time += time_taken;
        println!("\t{:?} in {:.2?}.", source_path.file_name().unwrap(), time_taken);

        file_count += 1;
    }

    println!("Total time taken: {:.2?}.", total_time);

    Ok(file_count)
}

fn merge_src_file(source_path: &PathBuf, out_path: &PathBuf, lib_dir: &PathBuf) -> Result<Duration, Box<dyn Error>> {
    let now = Instant::now();

    let mut document: String = fs::read_to_string(source_path)?;

    let mut includes = get_file_includes(&document);

    // Clone to not overwrite values being looped
    for file in includes.clone() {
        let lib_path = Path::new(&lib_dir).join(&file);
        includes.extend(get_lib_dependencies(&lib_path, "")?);
    }

    let mut unique_includes: Vec<String> = Vec::new();
    for file in &includes {
        if !unique_includes.contains(file) {
            unique_includes.push(file.to_string());
        }
    }

    for file in unique_includes {
        let lib_path = Path::new(&lib_dir).join(&file);
        // The same libraries will likely get read over and over again currently. Would be better to have a file cache
        let lib_content = fs::read_to_string(lib_path)?;
        let lib_content_clear = INCLUDE_REG.replace_all(&lib_content, "");

        document.push_str(&format!("\r\n# Source: {}\r\n", &file));
        document.push_str(&lib_content_clear);
    }

    fs::write(out_path, document)?;

    let elapsed = now.elapsed();
    Ok(elapsed)
}

fn get_lib_dependencies(lib_path: &PathBuf, parent_lib: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let lib_dir = lib_path.parent().unwrap();
    let lib_name = lib_path.file_name().unwrap().to_str().unwrap();

    let document = fs::read_to_string(lib_path)?;
    let mut includes = get_file_includes(&document);

    // Loop prevention
    if includes.iter().any(|inc| inc.as_str() == parent_lib) {
        return Err(format!("{:?} includes library {:?} that includes {:?}", parent_lib, lib_name, parent_lib).into());
    }

    for file in includes.clone() {
        let recursive_lib_path = Path::new(&lib_dir).join(&file);
        let recursive_dependencies = get_lib_dependencies(&recursive_lib_path, lib_name)?;

        includes.extend(recursive_dependencies);
    }

    Ok(includes)
}

fn get_file_includes(file: &str) -> Vec<String> {
    file.lines()
    .filter(|line| line.starts_with("#include "))
    .map(|line| line.replace("#include ", ""))
    .collect::<Vec<_>>()
}
