use anyhow::Result;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, PathBuf};

pub fn user_home() -> PathBuf {
    let home = env::var("HOME").expect("should have a home");
    PathBuf::from(home)
}

pub fn app_home() -> PathBuf {
    let home = env::var("HOME").expect("should have a home");

    let abs_home = fs::canonicalize(home.as_str()).expect("home should have an absolute path");
    let full_home = abs_home.as_path().join(".ngmailer");

    full_home
}

pub fn sub_folders(app_home: PathBuf) -> Vec<PathBuf> {
    let list = [
        app_home.as_path().join("queue"),
        app_home.as_path().join("sent"),
        app_home.as_path().join("errors"),
        app_home.as_path().join("logs"),
        app_home.as_path().join("templates"),
    ];

    Vec::from(list)
}

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

    #[test]
    fn test_sub_folders() {
        let home = PathBuf::from("tests");
        let folders = sub_folders(home);

        println!("subs: {:?}", folders);
        assert_eq!(folders.len(), 5);

        for path in folders {
            assert!(path.exists());
        }
    }

    #[test]
    fn test_user_home() {
        let home = user_home();
        println!("user home: {}", home.display());
    }

    #[test]
    fn test_app_home() {
        let home = app_home();
        println!("app home: {}", home.display());
    }

    #[test]
    fn test_move_to_sent() {
        // remove the file first?
        let filename = "tests/queue/sent-message.toml";
        let (frompath, topath) = rename_from_to(filename, "sent");
        println!("from: {} to: {}", frompath.display(), topath.display());
        let resp = fs::remove_file(topath.clone());
        println!("{:?}", resp);

        let resp = move_file(frompath.clone(), topath.clone()).unwrap();

        println!("{:?}", resp);

        // now move it back
        let resp = move_file(topath.clone(), frompath.clone()).unwrap();
        println!("{:?}", resp);
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
