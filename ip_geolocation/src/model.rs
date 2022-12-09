use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
    pub country_name: String,
}
