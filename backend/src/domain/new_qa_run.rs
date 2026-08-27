use uuid::Uuid;

use crate::{domain::{FigmaFileKey, FigmaNodeId, TargetUrl}, routes::CreateRunRequest};

pub struct NewQaRun {
    pub project_id: Uuid,
    pub target_url: TargetUrl,
    pub figma_file_key: FigmaFileKey,
    pub figma_node_id: FigmaNodeId,
}

pub enum NewQaRunError {
 TargetUrlError(String),
 FigmaFileKeyError(String),
 FigmaNodeIdError(String)
}

impl TryFrom<CreateRunRequest> for NewQaRun {
 type Error = NewQaRunError;
  fn try_from(value: CreateRunRequest) -> Result<Self, Self::Error> {
     let target_url = TargetUrl::parse(value.target_url).map_err(NewQaRunError::TargetUrlError)?;

     let figma_file_key = FigmaFileKey::parse(value.figma_file_key).map_err(NewQaRunError::FigmaFileKeyError)?;

     let figma_node_id = FigmaNodeId::parse(value.figma_node_id).map_err(NewQaRunError::FigmaNodeIdError)?;

     Ok(Self {
            project_id: value.project_id,
            target_url,
            figma_file_key,
            figma_node_id,
        })

 }
}