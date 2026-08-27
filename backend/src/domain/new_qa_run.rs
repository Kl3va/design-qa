use uuid::Uuid;

use crate::{domain::{FigmaFileKey, FigmaNodeId, TargetUrl}, routes::CreateRunRequest};

pub struct NewQaRun {
    pub project_id: Uuid,
    pub target_url: TargetUrl,
    pub figma_file_key: FigmaFileKey,
    pub figma_node_id: FigmaNodeId,
}


#[derive(Debug, thiserror::Error)]
pub enum NewQaRunError {
    #[error("Invalid target URL: {0}")]
    TargetUrlError(String),

    #[error("Invalid Figma file key: {0}")]
    FigmaFileKeyError(String),

    #[error("Invalid Figma node id: {0}")]
    FigmaNodeIdError(String),
}

impl TryFrom<(CreateRunRequest, Uuid)> for NewQaRun {
 type Error = NewQaRunError;
  fn try_from(value: (CreateRunRequest, Uuid)) -> Result<Self, Self::Error> {
       let (req, project_id) = value;
     let target_url = TargetUrl::parse(req.target_url).map_err(NewQaRunError::TargetUrlError)?;

     let figma_file_key = FigmaFileKey::parse(req.figma_file_key).map_err(NewQaRunError::FigmaFileKeyError)?;

     let figma_node_id = FigmaNodeId::parse(req.figma_node_id).map_err(NewQaRunError::FigmaNodeIdError)?;

     Ok(Self {
            project_id,
            target_url,
            figma_file_key,
            figma_node_id,
        })

 }
}