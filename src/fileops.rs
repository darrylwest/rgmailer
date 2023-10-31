use anyhow::Result;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, PathBuf};

// finds the absolute path; substibutes the filenames parent with to_target
pub fn rename_from_to(filename: &str, to_target: &str) -> (PathBuf, PathBuf) {
    let path = PathBuf::from(filename);
    let abs_path = fs::canonicalize(path).unwrap();

    let mut parts: Vec<_> = abs_path.components().collect();

    // find the name and subfolder
    let idx = parts.len() - 2;
    println!("{:?}", parts[idx]);
    parts[idx] = Component::Normal(OsStr::new(to_target));

    let to_path = parts.iter().collect();

    (abs_path, to_path)
}

pub fn move_file(from: PathBuf, to: PathBuf) -> Result<()> {
    match fs::rename(from, to) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // create/call a function to build the test home strucure

    #[test]
    fn test_move_to_sent() {
        // remove the file first?
        let filename = "tests/queue/sent-message.toml";
        let (frompath, topath) = rename_from_to(filename, "sent");
        println!("from: {} to: {}", frompath.display(), topath.display());
        let resp = fs::remove_file(topath.clone());
        println!("{:?}", resp);

        let resp = move_file(frompath, topath.clone()).unwrap();

        println!("{:?}", resp);

        let _ = fs::remove_file(topath.clone());
    }

    #[test]
    fn test_rename_from_to() {
        let filename = "tests/queue/7mNdj105Ch0c.toml";

        let (frompath, topath) = rename_from_to(filename, "sent");

        println!("from: {} to: {}", frompath.display(), topath.display());

        assert!(frompath.ends_with("rgmailer/tests/queue/7mNdj105Ch0c.toml"));
        assert!(topath.ends_with("rgmailer/tests/sent/7mNdj105Ch0c.toml"));
    }
}
