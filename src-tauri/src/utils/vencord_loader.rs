use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

const VENCORD_RELEASE_URL: &str =
    "https://api.github.com/repos/Vendicated/Vencord/releases/latest";
const VENCORD_FILES: &[&str] = &["patcher.js", "preload.js", "renderer.js"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VencordVersion {
    pub tag_name: String,
    pub assets: Vec<VencordAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VencordAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub struct VencordLoader {
    data_dir: PathBuf,
    vencord_dir: PathBuf,
}

impl VencordLoader {
    pub fn new(data_dir: PathBuf) -> Self {
        let vencord_dir = data_dir.join("vencord");
        Self {
            data_dir,
            vencord_dir,
        }
    }

    pub fn vencord_dir(&self) -> &PathBuf {
        &self.vencord_dir
    }

    pub async fn ensure_vencord_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.vencord_dir).await?;

        let version_file = self.vencord_dir.join(".version");
        let current_version = if version_file.exists() {
            fs::read_to_string(&version_file).await.ok()
        } else {
            None
        };

        let latest_version = self.get_latest_version().await?;

        if current_version.as_deref() == Some(&latest_version) {
            log::info!("Vencord is up to date: {}", latest_version);
            return Ok(());
        }

        log::info!("Downloading Vencord {}", latest_version);
        self.download_vencord().await?;

        fs::write(&version_file, &latest_version).await?;
        log::info!("Vencord updated to {}", latest_version);

        Ok(())
    }

    async fn get_latest_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Veskto")
            .build()?;

        let response = client
            .get(VENCORD_RELEASE_URL)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        let release: VencordVersion = response.json().await?;
        Ok(release.tag_name)
    }

    async fn download_vencord(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Veskto")
            .build()?;

        let response = client
            .get(VENCORD_RELEASE_URL)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        let release: VencordVersion = response.json().await?;

        for asset_name in VENCORD_FILES {
            if let Some(asset) = release.assets.iter().find(|a| a.name == *asset_name) {
                let dest = self.vencord_dir.join(asset_name);
                let content = client.get(&asset.browser_download_url).send().await?.bytes().await?;
                fs::write(&dest, &content).await?;
                log::info!("Downloaded {}", asset_name);
            }
        }

        Ok(())
    }

    pub async fn get_injection_script(&self) -> Result<String, Box<dyn std::error::Error>> {
        let renderer_path = self.vencord_dir.join("renderer.js");
        if renderer_path.exists() {
            let content = fs::read_to_string(&renderer_path).await?;
            Ok(content)
        } else {
            Ok(String::new())
        }
    }
}
