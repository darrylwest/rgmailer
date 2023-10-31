use anyhow::Result;
use std::fs;
// use std::io;
use std::ffi::OsStr;
use std::path::{Component, PathBuf};

// finds the absolute path; substibutes the filenames parent with to_target
pub fn rename_from_to(filename: &str, to_target: &str) -> PathBuf {
    let path = PathBuf::from(filename);
    let abs_path = fs::canonicalize(path).unwrap();

    let mut parts: Vec<_> = abs_path.components().collect();

    // find the name and subfolder
    let idx = parts.len() - 2;
    println!("{:?}", parts[idx]);
    parts[idx] = Component::Normal(OsStr::new(to_target));

    let to_path = parts.iter().collect();

    to_path
}

pub fn move_to_sent(_filename: &str) -> Result<()> {
    Ok(())
}

pub fn move_to_errors(_filename: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // create/call a function to build the test home strucure

    #[test]
    fn test_rename_from_to() {
        let filename = "tests/queue/7mNdj105Ch0c.toml";

        let path = rename_from_to(filename, "sent");

        println!("path: {}", path.display());

        assert!(path.ends_with("tests/sent/7mNdj105Ch0c.toml"));
    }
}
