use aqs_analysis::network::MonitoringNetwork;
use aqs_analysis::station::Station;

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
    let mut station1 = create_test_station("01-001-0001", "Station1", 40.0, -74.0);
    let mut station2 = create_test_station("01-001-0002", "Station2", 40.1, -74.1);
    let mut station3 = create_test_station("01-001-0003", "Station3", 40.2, -74.2);
    
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
    let mut network = MonitoringNetwork::new();
    let mut pollution_data = std::collections::HashMap::new();
    
    // Create test stations with perfectly correlated isolation and pollution
    // (isolation increases as pollution increases)
    for i in 0..10 {
        let id = format!("01-001-{:04}", i);
        let lat = 40.0 + (i as f64 * 0.1);
        let lon = -74.0 - (i as f64 * 0.1);
        
        let mut station = create_test_station(&id, &format!("Station{}", i), lat, lon);
        network.add_station(station);
        
        // Add perfectly correlated pollution data (higher for more isolated stations)
        pollution_data.insert(id, 10.0 + i as f64);
    }
    
    // Build adjacency list and calculate isolation
    network.build_adjacency_list();
    network.calculate_isolation(3);
    
    // Calculate correlation
    let correlation = network.analyze_correlation(&pollution_data);
    
    // Correlation should be very close to 1.0 (perfect positive correlation)
    assert!(correlation > 0.95, "Correlation calculation error: {}", correlation);
}

// Helper function to create test stations
fn create_test_station(id: &str, name: &str, lat: f64, lon: f64) -> Station {
    let mut station = Station {
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
    };
    station
}