use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Geolocation {
    pub latitude: f32,
    pub longitude: f32,
    pub country_name: String,
}
