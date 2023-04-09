use std::path::{Path, PathBuf};

use anyhow::{bail, Context};

/// find similar sounding files to `file` in `dir`
fn similar_files_to(dir: &Path, file: &str) -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    let main_file = file.to_ascii_lowercase();
    let files = std::fs::read_dir(dir).context("Failed to read dir")?;

    let mut result = Vec::new();

    for file in files {
        let file = file.context("Failed to read file")?;
        let path = file.path();
        let file = path
            .file_stem()
            .context("file without stem")?
            .to_ascii_lowercase();

        let file = file.to_str().context("unusual file name")?;

        let stars_with = file.starts_with(main_file.as_str());

        if stars_with {
            result.push(path);
        }
    }

    Ok(result.into_iter())
}

pub fn obtain_db(chip: &str) -> anyhow::Result<PathBuf> {
    let db = obtain_db_folder()?;

    let similar: Vec<_> = similar_files_to(&db, chip)
        .context("No db found")?
        .take(10)
        .collect();

    if similar.len() == 1 {
        let path = similar.into_iter().next().unwrap();
        Ok(path)
    } else {
        bail!("Found multiple similar files: {:?}", similar);
    }
}

pub fn obtain_db_folder() -> anyhow::Result<PathBuf> {
    let paths = db_paths()?;
    for path in paths {
        if path.exists() {
            return Ok(path);
        }
    }
    bail!("No db found")
}

/// get potential db paths
fn db_paths() -> anyhow::Result<Vec<PathBuf>> {
    if cfg!(windows) {
        return Ok(db_paths_windows());
    }

    if cfg!(unix) {
        return Ok(db_paths_unix());
    }

    bail!("Unsupported OS")
}

fn db_paths_windows() -> Vec<PathBuf> {
    let paths = vec![r#"C:\Program Files (x86)\STMicroelectronics\STM32Cube\STM32CubeMX\db\mcu"#];

    paths.into_iter().map(|p| PathBuf::from(p)).collect()
}

fn db_paths_unix() -> Vec<PathBuf> {
    let paths = vec!["/Applications/STMicroelectronics/STM32CubeMX.app/Contents/Resources/db/mcu"];

    paths.into_iter().map(|p| PathBuf::from(p)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_exists() {
        let db = obtain_db_folder().unwrap();
        assert!(db.exists());
    }
}
