use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use site_gen::get_pages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pages = get_pages();

    let root_path = "./site";
    for page in pages {
        let path = if page.route.ends_with('/') {
            &format!("{}index.html", page.route)
        } else {
            &page.route
        };

        let path = append_path(PathBuf::from(root_path), &path);
        let tmp_path = append_path(path.clone(), "_tmp");

        let file = File::create(tmp_path.clone())?;
        let mut file = BufWriter::new(file);

        file.write_all(page.content.as_bytes())?;
        file.flush()?;

        if std::fs::exists(path.clone())? {
            std::fs::remove_file(path.clone())?;
        }
        std::fs::rename(tmp_path, path)?;
    }

    let styles_path = append_path(PathBuf::from(root_path), "/styles/");

    let css_ext = "css";
    let grass_options = grass::Options::default();
    let entries = std::fs::read_dir(styles_path)?;
    for entry in entries {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_file() {
                let entry_path = entry.path();
                let ext = entry_path.extension().unwrap().to_str().unwrap();

                if ext == "scss" || ext == "sass" {
                    let result = grass::from_path(&entry_path, &grass_options).unwrap();

                    let stem = entry_path.file_stem().unwrap().to_str().unwrap();

                    let mut new_path = entry_path.clone();
                    new_path.set_file_name(format!("{}.{}", stem, css_ext));
                    let file = File::create(new_path)?;
                    let mut file = BufWriter::new(file);
                    file.write_all(result.as_bytes())?;
                }
            }
        }
    }

    Ok(())
}

pub fn append_path(path: PathBuf, s: &str) -> PathBuf {
    let mut path = path.into_os_string();
    path.push(s);
    PathBuf::from(path)
}
