#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Auth {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
    pub created_at: i64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthWithDetails {
    #[serde(flatten)]
    pub auth: Auth,
    pub host: String,
    /// The client id of the API application.
    pub client_id: String,
    /// The client secret of the API application.
    pub client_secret: String,
}
