pub enum Loader {
    Vanilla,
}

/// Temporary
struct DownloadHandler {}

/// Temporary
impl portablemc::download::Handler for DownloadHandler {
    fn on_progress(&mut self, count: u32, total_count: u32, size: u32, total_size: u32) {
        println!("Download Handler lmao")
    }
}

/// Temporary
pub struct GameVersion {
    pub name: String,
}

/// Temporary
impl From<portablemc::moj::ManifestVersion<'_>> for GameVersion {
    fn from(value: portablemc::moj::ManifestVersion) -> Self {
        Self {
            name: value.name().into(),
        }
    }
}

pub fn search(query: &str, loader: &Option<Loader>) -> crate::Result<Vec<GameVersion>> {
    let mut handler = DownloadHandler {};
    let manifest = portablemc::moj::Manifest::request(&mut handler)?;

    Ok(manifest.iter().map(GameVersion::from).collect())
}
