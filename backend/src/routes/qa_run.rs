use uuid::Uuid;

pub struct CreateRunRequest {
    pub project_id: Uuid,
    pub target_url: String,
    pub figma_file_key: String,
    pub figma_node_id: String,
}