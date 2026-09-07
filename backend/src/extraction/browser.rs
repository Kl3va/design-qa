use serde::Deserialize;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum BrowserExtractError {
    #[error("failed to spawn extractor process: {0}")]
    Spawn(#[source] std::io::Error),

    #[error("extractor process exited with status {0}")]
    NonZeroExit(i32),

    #[error("failed to read extractor output: {0}")]
    ReadOutput(#[source] std::io::Error),

    #[error("failed to parse extractor output: {0}")]
    InvalidJson(#[source] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct RawDomExtract {
    pub url: String,
    pub viewport: RawViewport,
    #[serde(rename = "totalRelevantVisible")]
    pub total_relevant_visible: usize,
    pub root: RawDomNode,
}

#[derive(Debug, Deserialize)]
pub struct RawViewport {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
pub struct RawFont {
    pub family: String,
    pub size: String,
    pub weight: String,
    pub style: String,
    #[serde(rename = "lineHeight")]
    pub line_height: String,
    #[serde(rename = "letterSpacing")]
    pub letter_spacing: String,
    #[serde(rename = "textTransform")]
    pub text_transform: String,
    #[serde(rename = "textAlign")]
    pub text_align: String,
    #[serde(rename = "textDecorationLine")]
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
pub struct RawBorderRadius {
    #[serde(rename = "topLeft")]
    pub top_left: String,
    #[serde(rename = "topRight")]
    pub top_right: String,
    #[serde(rename = "bottomRight")]
    pub bottom_right: String,
    #[serde(rename = "bottomLeft")]
    pub bottom_left: String,
}

#[derive(Debug, Deserialize)]
pub struct RawLayout {
    pub display: String,
    pub position: String,
    #[serde(rename = "zIndex")]
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
pub struct RawDomNode {
    pub id: i64,
    pub tag: String,
    #[serde(rename = "idAttr")]
    pub id_attr: Option<String>,
    #[serde(rename = "className")]
    pub class_name: Option<String>,
    pub selector: String,
    pub role: Option<String>,
    pub text: String,
    #[serde(rename = "ownText")]
    pub own_text: String,
    #[serde(rename = "hasText")]
    pub has_text: bool,
    pub rect: RawRect,
    #[serde(rename = "inViewport")]
    pub in_viewport: bool,
    pub font: RawFont,
    pub color: String,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    pub opacity: String,
    pub border: RawBorderSides,
    #[serde(rename = "borderRadius")]
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
    let tmp_dir = std::env::temp_dir().join(format!("design-qa-{}", uuid::Uuid::new_v4()));

    let status = Command::new("node")
        .arg("extract.mjs")
        .arg(target_url)
        .arg(format!("--out={}", tmp_dir.display()))
        .current_dir(extractor_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status()
        .await
        .map_err(BrowserExtractError::Spawn)?;

    if !status.success() {
        return Err(BrowserExtractError::NonZeroExit(status.code().unwrap_or(-1)));
    }

    let json_path = tmp_dir.join("extract.json");
    let contents = tokio::fs::read_to_string(&json_path)
        .await
        .map_err(BrowserExtractError::ReadOutput)?;

    serde_json::from_str(&contents).map_err(BrowserExtractError::InvalidJson)
}