use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

// pub async fn create_project () -> Result<(), std::io::Error> {

// }
