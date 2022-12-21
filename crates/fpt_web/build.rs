use anyhow::{anyhow, Result};
use minify_html_onepass as mini;
use static_files::resource_dir;
use std::{fs, path::Path};

fn minify_files(
    from_directory: &str,
    to_directory: &str,
    current_path: Option<&str>,
) -> Result<()> {
    let current_path = current_path.unwrap_or(from_directory);

    if !Path::new(to_directory).try_exists()? {
        fs::create_dir_all(to_directory)?;
    }

    if Path::new(current_path).is_dir() {
        // Recursivly call `minify_files` on directory content.
        for entry in fs::read_dir(current_path)? {
            let entry = entry?.path();
            let new_current_path = entry
                .to_str()
                .ok_or_else(|| anyhow!("failed reading path"))?;
            minify_files(from_directory, to_directory, Some(new_current_path))?;
        }
    } else {
        let new_path = current_path.replacen(from_directory, to_directory, 1);
        let new_path = Path::new(&new_path);

        // Create subdirectories if needed.
        if let Some(parent) = new_path.parent() {
            if !parent.try_exists()? {
                fs::create_dir_all(parent)?;
            }
        }

        // Minify.
        let out = match new_path.extension().and_then(|x| x.to_str()) {
            Some("html") | Some("css") => {
                let data = fs::read(current_path)?;
                mini::copy(&data, &mini::Cfg::new()).map_err(|e| anyhow!("{:?}", e))?
            }
            _ => fs::read(current_path)?,
        };

        fs::write(new_path, out)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    minify_files("www", "www_mini", None)?;
    resource_dir("www_mini").build()?;

    Ok(())
}
