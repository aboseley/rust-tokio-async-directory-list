use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct LogFileNames(Vec<String>);

impl LogFileNames {
    pub fn new() -> LogFileNames {
        LogFileNames(vec![])
    }
}

fn is_logfile(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .map(|s| s == "log")
        .unwrap_or(false)
}

fn get_log_filenames() -> Result<Vec<String>> {
    Ok(std::fs::read_dir("/tmp")?
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_logfile)
        .map(|d| d.path().display().to_string())
        .collect())
}

fn have_log_name(entry: DirEntry) -> Option<String> {
    match entry.path().extension() {
        Some(ext) => {
            if (ext == "log") {
                Some(entry.path().display().to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_log_filenames2() -> Result<Vec<String>> {
    Ok(std::fs::read_dir("/tmp")?
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(have_log_name)
        .collect())
}

fn get_log_filenames3() -> Result<Vec<String>> {
    // flatmap is more general than filter_map
    Ok(std::fs::read_dir("/tmp")?
        .into_iter()
        .filter_map(Result::ok)
        .flat_map(have_log_name)
        .collect())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    if let Ok(log_files) = tokio::task::spawn_blocking(get_log_filenames).await? {
        let log_files_json = serde_json::to_string(&log_files).unwrap();
        println!("{}", log_files_json);
    }

    return Ok(());
}
