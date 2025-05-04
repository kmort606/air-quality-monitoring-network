//module for representing air quality monitoring stations and their attributes


use serde::Deserialize;

//represents an air quality monitoring station with its metadata and isolation metrics
//used as nodes in the monitoring network graph
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
    //generate a unique id by combining state, county, and site codes
    //this creates a standard format used for consistent identification
    pub fn generate_id(&mut self) {
        self.id = format!("{}-{}-{}", self.state_code, self.county_code, self.site_number);
    }
}