use anyhow::Result;
use log::{error, info};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// the user's home folder
pub fn user_home() -> PathBuf {
    let home = env::var("HOME").expect("should have a home");
    PathBuf::from(home)
}

/// the application home folder
pub fn app_home() -> PathBuf {
    let home = env::var("HOME").expect("should have a home");

    let abs_home = fs::canonicalize(home.as_str()).expect("home should have an absolute path");
    let full_home = abs_home.as_path().join(".rgmailer");

    info!("rgmailer home is {}", full_home.display());

    full_home
}

/// list of sub-folders for the rgmailer application
pub fn sub_folders(app_home: PathBuf) -> Vec<PathBuf> {
    let list = [
        app_home.as_path().join("queue"),
        app_home.as_path().join("sent"),
        app_home.as_path().join("errors"),
        app_home.as_path().join("logs"),
        app_home.as_path().join("templates"),
        app_home.as_path().join("config"),
    ];

    Vec::from(list)
}

/// initialize the file structure
pub fn init_folders(app_home: PathBuf) -> Result<()> {
    let folders = sub_folders(app_home);
    for folder in folders {
        if !folder.exists() {
            let msg = format!("creating: {}", folder.display());
            match fs::create_dir_all(folder) {
                Ok(_) => info!("{}", msg),
                Err(e) => {
                    error!("error: {} {}", e, msg);
                    return Err(e.into());
                }
            }
        }
    }

    Ok(())
}

/// create a timestamp and mv the file to the sent folder with the ts
pub fn move_to_sent(filename: &str) -> Result<()> {
    info!("move {} to sent folder", filename);
    let now = SystemTime::now().duration_since(UNIX_EPOCH);
    let ts = now.unwrap().as_millis();

    let path = PathBuf::from(filename);
    let npath = path.file_name().unwrap().to_str().unwrap();
    let ext = format!("-{}.toml", ts);
    let npath = npath.to_string().replace(".toml", &ext);
    let npath = PathBuf::from(format!("sent/{}", npath));
    move_file(path, npath)
}

/// finds the absolute path; substibutes the filenames parent with to_target
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

/// move a fiel from -> to
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
        let folders = sub_folders(home.clone());

        println!("subs: {:?}", folders);
        assert_eq!(folders.len(), 6);

        init_folders(home.clone()).unwrap();

        for path in folders {
            assert!(path.exists());
        }
    }

    #[test]
    fn test_init_new_home() {
        let home = PathBuf::from("tmp");
        let folders = sub_folders(home.clone());

        init_folders(home.clone()).unwrap();

        for path in folders {
            assert!(path.exists());
        }

        fs::remove_dir_all("tmp").expect("should remove tmp folders");
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
