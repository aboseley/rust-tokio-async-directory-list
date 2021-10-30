use anyhow::Result;
use std::path::Path;
use tokio_stream::StreamExt;

async fn get_filenames(in_path: &Path, with_ext: &str) -> Result<Vec<String>> {
    use tokio::fs;
    use tokio_stream::wrappers::ReadDirStream;

    let matching_extension = |entry: &fs::DirEntry| -> bool {
        entry
            .path()
            .extension()
            .map(|s| s == with_ext)
            .unwrap_or(false)
    };

    fn to_filenames(entry: fs::DirEntry) -> String {
        entry.path().display().to_string()
    }

    let dir = fs::read_dir(in_path).await?;
    let dir_stream = ReadDirStream::new(dir);

    Ok(dir_stream
        .filter_map(Result::ok)
        .filter(matching_extension)
        .map(to_filenames)
        .collect()
        .await)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filenames = get_filenames(Path::new("/tmp"), "log").await?;
    let filenames_as_json = serde_json::to_string(&filenames)?;
    println!("{}", filenames_as_json);

    return Ok(());
}
