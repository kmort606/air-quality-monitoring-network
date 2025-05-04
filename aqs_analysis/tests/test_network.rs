use aqs_analysis::network::MonitoringNetwork;
use aqs_analysis::station::Station;

mod test_helpers {
    use super::Station;
    
    pub fn create_test_station(id: &str, name: &str, lat: f64, lon: f64) -> Station {
        Station {
            id: id.to_string(),
            state_code: "01".to_string(),
            county_code: "001".to_string(),
            site_number: id.split("-").last().unwrap_or("0001").to_string(),
            latitude: lat,
            longitude: lon,
            land_use: "RESIDENTIAL".to_string(),
            location_setting: "URBAN".to_string(),
            site_name: name.to_string(),
            state_name: "Test State".to_string(),
            county_name: "Test County".to_string(),
            city_name: "Test City".to_string(),
            avg_distance_to_neighbors: None,
        }
    }
}

#[test]
fn test_haversine_distance() {
    let network = MonitoringNetwork::new();
    
    // New York City to Los Angeles distance (about 3940 km)
    let nyc_lat = 40.7128;
    let nyc_lon = -74.0060;
    let la_lat = 34.0522;
    let la_lon = -118.2437;
    
    let distance = network.haversine_distance(nyc_lat, nyc_lon, la_lat, la_lon);
    
    // Check that the distance is within 1% of expected value
    assert!((distance - 3940.0).abs() < 40.0, "Distance calculation error: {}", distance);
}

#[test]
fn test_calculate_isolation() {
    let mut network = MonitoringNetwork::new();
    
    // Create test stations
    let station1 = test_helpers::create_test_station("01-001-0001", "Station1", 40.0, -74.0);
    let station2 = test_helpers::create_test_station("01-001-0002", "Station2", 40.1, -74.1);
    let station3 = test_helpers::create_test_station("01-001-0003", "Station3", 40.2, -74.2);
    
    // Add stations to network
    network.add_station(station1);
    network.add_station(station2);
    network.add_station(station3);
    
    // Build adjacency list
    network.build_adjacency_list();
    
    // Calculate isolation with k=1
    network.calculate_isolation(1);
    
    // Check that all stations have isolation values
    for (_, station) in &network.stations {
        assert!(station.avg_distance_to_neighbors.is_some(), 
                "Station missing isolation value: {}", station.site_name);
    }
}

#[test]
fn test_correlation_calculation() {
    // Instead of testing with dynamically calculated isolation values,
    // let's create a simpler test with hardcoded data that we control completely
    let mut network = MonitoringNetwork::new();
    let mut pollution_data = std::collections::HashMap::new();
    
    // Create just two stations with fixed IDs
    let station1_id = "01-001-0001".to_string();
    let station2_id = "01-001-0002".to_string();
    
    // Create and add the first station
    let mut station1 = Station {
        id: station1_id.clone(),
        state_code: "01".to_string(),
        county_code: "001".to_string(),
        site_number: "0001".to_string(),
        latitude: 40.0,
        longitude: -74.0,
        land_use: "RESIDENTIAL".to_string(),
        location_setting: "URBAN".to_string(),
        site_name: "Station1".to_string(),
        state_name: "Test State".to_string(),
        county_name: "Test County".to_string(),
        city_name: "Test City".to_string(),
        // Set isolation value directly instead of calculating it
        avg_distance_to_neighbors: Some(10.0),
    };
    
    // Create and add the second station
    let mut station2 = Station {
        id: station2_id.clone(),
        state_code: "01".to_string(),
        county_code: "001".to_string(),
        site_number: "0002".to_string(),
        latitude: 40.1,
        longitude: -74.1,
        land_use: "RESIDENTIAL".to_string(),
        location_setting: "URBAN".to_string(),
        site_name: "Station2".to_string(),
        state_name: "Test State".to_string(),
        county_name: "Test County".to_string(),
        city_name: "Test City".to_string(),
        // Set isolation value directly instead of calculating it
        avg_distance_to_neighbors: Some(20.0),
    };
    
    // Add stations to network
    network.add_station(station1);
    network.add_station(station2);
    
    // Set pollution values that have perfect negative correlation with isolation
    // Station1: isolation=10, pollution=20
    // Station2: isolation=20, pollution=10
    pollution_data.insert(station1_id, 20.0);
    pollution_data.insert(station2_id, 10.0);
    
    // Calculate correlation
    let correlation = network.analyze_correlation(&pollution_data);
    
    // With exactly two data points with perfect negative correlation,
    // we should get exactly -1.0
    assert!(correlation < -0.9, "Expected strong negative correlation, got: {}", correlation);
}