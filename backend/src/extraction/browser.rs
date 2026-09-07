use serde::Deserialize;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;
use tokio::process::Command;

const EXTRACT_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Debug, thiserror::Error)]
pub enum BrowserExtractError {
    #[error("failed to create temp directory: {0}")]
    TempDir(#[source] std::io::Error),

    #[error("failed to spawn extractor process: {0}")]
    Spawn(#[source] std::io::Error),

    #[error("extractor process timed out")]
    Timeout,

    #[error("extractor process exited with status {code}: {stderr}")]
    NonZeroExit { code: i32, stderr: String },

    #[error("failed to wait on extractor process: {0}")]
    Wait(#[source] std::io::Error),

    #[error("failed to parse extractor output: {0}")]
    InvalidJson(#[source] serde_json::Error),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawDomExtract {
    pub url: String,
    pub viewport: RawViewport,
    pub total_relevant_visible: usize,
    pub root: RawDomNode,
}

#[derive(Debug, Deserialize)]
pub struct RawViewport {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawFont {
    pub family: String,
    pub size: String,
    pub weight: String,
    pub style: String,
    pub line_height: String,
    pub letter_spacing: String,
    pub text_transform: String,
    pub text_align: String,
    pub text_decoration_line: String,
}

#[derive(Debug, Deserialize)]
pub struct RawBorderSides {
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub left: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawBorderRadius {
    pub top_left: String,
    pub top_right: String,
    pub bottom_right: String,
    pub bottom_left: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawLayout {
    pub display: String,
    pub position: String,
    pub z_index: String,
}

#[derive(Debug, Deserialize)]
pub struct RawRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawDomNode {
    pub id: i64,
    pub tag: String,
    pub id_attr: Option<String>,
    pub class_name: Option<String>,
    pub selector: String,
    pub role: Option<String>,
    pub text: String,
    pub own_text: String,
    pub has_text: bool,
    pub rect: RawRect,
    pub in_viewport: bool,
    pub font: RawFont,
    pub color: String,
    pub background_color: String,
    pub opacity: String,
    pub border: RawBorderSides,
    pub border_radius: RawBorderRadius,
    pub layout: RawLayout,
    #[serde(default)]
    pub children: Vec<RawDomNode>,
}

#[tracing::instrument(name = "Run browser extractor", skip(extractor_dir))]
pub async fn extract_dom(
    target_url: &str,
    extractor_dir: &std::path::Path,
) -> Result<RawDomExtract, BrowserExtractError> {
    // This doesn't actively do anything with the screenshot till i decide where to move it or structure tied to run_id
    let tmp_dir = TempDir::new().map_err(BrowserExtractError::TempDir)?;

    let spawn_and_wait = async {
        let child = Command::new("node")
            .arg("extract.mjs")
            .arg(target_url)
            .arg(format!("--out={}", tmp_dir.path().display()))
            .current_dir(extractor_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(BrowserExtractError::Spawn)?;

        child
            .wait_with_output()
            .await
            .map_err(BrowserExtractError::Wait)
    };

    let output = tokio::time::timeout(EXTRACT_TIMEOUT, spawn_and_wait)
        .await
        .map_err(|_| BrowserExtractError::Timeout)??;

    if !output.status.success() {
        return Err(BrowserExtractError::NonZeroExit {
            code: output.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    // extract.mjs's own logging goes to stderr, so stdout is pure JSON.
    serde_json::from_slice(&output.stdout).map_err(BrowserExtractError::InvalidJson)

   
}