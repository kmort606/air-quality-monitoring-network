use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use csv;
use serde::Deserialize;
use crate::station::Station;

//function to read the station data csv file using serde
pub fn read_stations<P:AsRef<Path>>(path: P) -> Result<Vec<Station>, Box <dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(true).from_reader(reader);
    let mut stations = Vec::new();

    //using serde deseralization (lecture 33)
    for result in csv_reader.deserialize::<Station>() {
        match result {
            Ok(mut station) => {
                //generate id after deserialization
                station.generate_id();
                stations.push(station);
            },
            Err(err) => {
                //log error but continue anyway
                eprintln!("Error deserializing station {}", err );
            }
        }
    }
    println!("loaded {} stations with valid coordinates", stations.len());
    Ok(stations)
}

//pollution measurement struct for deserialization
#[derive(Debug, Deserialize)]
struct PollutionMeasurement {
    #[serde(rename = "State Code")]
    state_code: String,
    #[serde(rename = "County Code")]
    county_code: String,
    #[serde(rename = "Site Num")]
    site_number: String,
    #[serde(rename = "Parameter Code")]
    parameter_code: String,
    #[serde(rename = "Sample Duration")]
    sample_duration: String,
    #[serde(rename = "Pollutant Standard")]
    pollutant_standard: String,
    #[serde(rename = "Arithmetic Mean")]
    arithmetic_mean: Option<f64>,
}

//function to read the poltion data from csv using serde
pub fn read_pollution<P: AsRef<Path>>(path: P) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(true).from_reader(reader);
    let mut pollution_data = HashMap::new();
    //next two lines:
    //focus on PM2.5 data, which is code 88101 (most important measure of air pollution, particulate matter of certain size)
    //and getting the 24 hour block average measurments for consistancy
    let target_parameter = "88101";
    let target_duration = "24-HR BLK AVG";

    //using serde deserialization
    for result in csv_reader.deserialize::<PollutionMeasurement>() {
        match result {
            Ok(measurement) => {
                //Check if this is a PM2.5 record with appropriate averaging time
                if measurement.parameter_code == target_parameter && 
                measurement.sample_duration == target_duration {
                    
                    //create a unique ID given a state, county, and site code
                    let id = format!("{}-{}-{}", 
                        measurement.state_code, 
                        measurement.county_code, 
                        measurement.site_number);
                    
                    //use arithmetic mean if available
                    if let Some(mean) = measurement.arithmetic_mean {
                        //use PM25 Annual measurement for consistent averaging
                        if measurement.pollutant_standard.contains("Annual") {
                            pollution_data.insert(id, mean);
                        } else if !pollution_data.contains_key(&id) {
                            //if we don't have the annual average, use any 24-hour average.
                            pollution_data.insert(id, mean);
                        }
                    }
                }
            },
            Err(err) => {
                //log the error but continue processing
                eprintln!("Error deserializing pollution data: {}", err);
            }
        }
    } 
    println!("Loaded pollution data for {} stations", pollution_data.len());
    
    Ok(pollution_data)
}