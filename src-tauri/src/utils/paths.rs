use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    if let Ok(data_dir) = std::env::var("VESKTO_DATA_DIR") {
        return PathBuf::from(data_dir);
    }

    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("veskto")
}

pub fn ensure_dir(path: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}
