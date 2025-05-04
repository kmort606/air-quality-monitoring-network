use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Station {
    #[serde(rename = "State Code")]
    pub state_code: String,
    #[serde(rename = "County Code")]
    pub county_code: String,
    #[serde(rename = "Site Number")]
    pub site_number: String,
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
    #[serde(rename = "Land Use")]
    pub land_use: String,
    #[serde(rename = "Location Setting")]
    pub location_setting: String,
    #[serde(rename = "Local Site Name")]
    pub site_name: String,
    #[serde(rename = "State Name")]
    pub state_name: String,
    #[serde(rename = "County Name")]
    pub county_name: String,
    #[serde(rename = "City Name")]
    pub city_name: String,
    #[serde(skip)]
    pub id: String,
    #[serde(skip)]
    pub avg_distance_to_neighbors: Option<f64>,
}

impl Station {
    // Generate a unique ID by combining state, county, and site codes
    pub fn generate_id(&mut self) {
        self.id = format!("{}-{}-{}", self.state_code, self.county_code, self.site_number);
    }
}