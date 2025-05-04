use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use crate::station::Station;

//struct to represent the graph network that connects the air quality monitors
pub struct MonitoringNetwork {
    pub stations: HashMap<String, Station>,
    pub adjacency_list: HashMap<String, Vec<(String, f64)>>, //station_id = (neighbor_id, distance)
}

//implementaton for graph
impl MonitoringNetwork {
    //create new empty network
    pub fn new() -> Self {
        MonitoringNetwork {
            stations: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }
    //to add a station to the network
    pub fn add_station(&mut self, station: Station) {
        let id = station.id.clone();
        self.stations.insert(id, station);
    }

    //calculate distance between two monitors given long and lat using Haversine distance formula
    pub fn haversine_distance(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        //constant for earth's radius in kilometers
        const R: f64 = 6371.0;
        //convert degrees to radians (necessary for equation)
        let lat1 = lat1.to_radians();
        let lon1 = lon1.to_radians();
        let lat2 = lat2.to_radians();
        let lon2 = lon2.to_radians();
        //haversine formula calculation
        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;
        let a = (dlat/2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon/2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());   
        R * c
    }
    //function to build adjacency list with all stations
    //node = station, edge = distance between two stations
    pub fn build_adjacency_list(&mut self) {
        // Only compute distances for stations within a reasonable range
        // For monitoring stations, stations more than 300km apart are unlikely to be relevant neighbors
        const MAX_NEIGHBOR_DISTANCE_KM: f64 = 300.0;
        
        // Optimization: Prefilter stations by rough geographic boundaries
        // Create a simple spatial index by dividing space into grid cells
        let mut spatial_index: HashMap<(i32, i32), Vec<String>> = HashMap::new();
        const GRID_SIZE_DEGREES: f64 = 1.0; // About 100km at the equator
        
        // Assign each station to a grid cell
        for (id, station) in &self.stations {
            let grid_x = (station.longitude / GRID_SIZE_DEGREES).floor() as i32;
            let grid_y = (station.latitude / GRID_SIZE_DEGREES).floor() as i32;
            spatial_index.entry((grid_x, grid_y)).or_insert(Vec::new()).push(id.clone());
        }
        
        println!("Created spatial index with {} cells", spatial_index.len());
        
        // Process each station
        let station_count = self.stations.len();
        let mut processed = 0;
        
        for (id1, station1) in &self.stations {
            let mut distances: Vec<(String, f64)> = Vec::new();
            
            // Get the grid cell for this station
            let grid_x = (station1.longitude / GRID_SIZE_DEGREES).floor() as i32;
            let grid_y = (station1.latitude / GRID_SIZE_DEGREES).floor() as i32;
            
            // Collect potential neighbors from 9 surrounding cells
            let mut potential_neighbors = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if let Some(stations_in_cell) = spatial_index.get(&(grid_x + dx, grid_y + dy)) {
                        potential_neighbors.extend(stations_in_cell.iter().cloned());
                    }
                }
            }
            
            // Calculate distances only to potential neighbors
            for id2 in potential_neighbors {
                if id1 == &id2 {
                    continue; // Skip self
                }
                
                let station2 = &self.stations[&id2];
                
                let distance = self.haversine_distance(
                    station1.latitude, station1.longitude,
                    station2.latitude, station2.longitude
                );
                
                // Only include neighbors within our distance threshold
                if distance <= MAX_NEIGHBOR_DISTANCE_KM {
                    distances.push((id2.clone(), distance));
                }
            }
            
            // Sort by distance
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
            
            // Store in adjacency list
            self.adjacency_list.insert(id1.clone(), distances);
        }
    }
    //function to calculate average distance to k nearest neighbors, measured as isolation
    pub fn calculate_isolation(&mut self, k: usize) {
        for (id, station) in &mut self.stations {
            if let Some(neighbors) = self.adjacency_list.get(id) {
                //get distances to k nearest neighbors
                let k_nearest: Vec<&(String, f64)> = neighbors.iter().take(k).collect();
                //calculate avg distance
                if !k_nearest.is_empty() {
                    let sum: f64 = k_nearest.iter().map(|(_, dist)| dist).sum();
                    let avg = sum / k_nearest.len() as f64;
                    //update station with new isolation metric
                    station.avg_distance_to_neighbors = Some(avg);
                }
            }
        }
    }
    //function to print isolation statistics 
    pub fn print_isolation_statistics(&self) {
        let mut isolation_values: Vec<f64> = Vec::new();
        for station in self.stations.values() {
            if let Some(isolation) = station.avg_distance_to_neighbors {
                isolation_values.push(isolation);
            }
        }
        
        if isolation_values.is_empty() {
            println!("No isolation values calculated");
            return;
        }
        
        isolation_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        
        let min = isolation_values[0];
        let max = isolation_values[isolation_values.len() - 1];
        let median = isolation_values[isolation_values.len() / 2];
        let mean: f64 = isolation_values.iter().sum::<f64>() / isolation_values.len() as f64;
        
        println!("Isolation statistics (km to 10 nearest neighbors):");
        println!("  Minimum: {:.2} km", min);
        println!("  Maximum: {:.2} km", max);
        println!("  Median: {:.2} km", median);
        println!("  Mean: {:.2} km", mean);
    }
    //correlation analysis between calculated isolation metric and pollution levels
    pub fn analyze_correlation(&self, pollution_data: &HashMap<String, f64>) -> f64 {
        //extract paired data points
        let mut pairs: Vec<(f64, f64)> = Vec::new();
        //for each station, if we know both isolation and polution, add pair to dataset
        for(id, station) in &self.stations {
            if let Some(isolation) = station.avg_distance_to_neighbors {
                if let Some(pollution) = pollution_data.get(id) {
                    pairs.push((isolation, *pollution));
                }
            }
        }
        //now calculate the correlation value (pearsons correlation)
        if pairs.is_empty() {
            return 0.0;
        }
        let n = pairs.len() as f64;
        let sum_x: f64 = pairs.iter().map(|(x, _)| x).sum();
        let sum_y: f64 = pairs.iter().map(|(_, y)| y).sum();
        let sum_xy: f64 = pairs.iter().map(|(x, y)| x * y).sum();
        let sum_xx: f64 = pairs.iter().map(|(x, _)| x * x).sum();
        let sum_yy: f64 = pairs.iter().map(|(_, y)| y * y).sum();
        
        let numerator = n * sum_xy - sum_x * sum_y;
        let denominator = ((n * sum_xx - sum_x * sum_x) * (n * sum_yy - sum_y * sum_y)).sqrt();
        
        if denominator == 0.0 {
            return 0.0;
        }
        
        numerator / denominator
    }
    //function to find potiental monitoring gaps (areas with high pollution but few nearby stations)
    pub fn find_monitoring_gaps(&self, pollution_data: &HashMap<String, f64>, 
                         isolation_threshold: f64, pollution_threshold: f64) -> Vec<(&Station, f64)> {
        let mut gaps = Vec::new();
        
        for (id, station) in &self.stations {
            //skip stations without isolation metrics or pollution data
            if let Some(isolation) = station.avg_distance_to_neighbors {
                if let Some(pollution) = pollution_data.get(id) {
                    //check if this station is in a high-pollution area but far from other stations
                    //thresholds set externally
                    if isolation > isolation_threshold && *pollution > pollution_threshold {
                        gaps.push((station, *pollution));
                    }
                }
            }
        }
        //sort by isolation (descending)
        gaps.sort_by(|(a, _), (b, _)| {
            b.avg_distance_to_neighbors.unwrap().partial_cmp(
                &a.avg_distance_to_neighbors.unwrap()).unwrap_or(Ordering::Equal)
        });
        
        gaps
    }
    //function to find and print the calculated monitoring gaps
    pub fn find_and_print_monitoring_gaps(&self, pollution_data: &HashMap<String, f64>) {
        //first get isolation stats
        let mut isolation_values: Vec<f64> = Vec::new();
        for station in self.stations.values() {
            if let Some(isolation) = station.avg_distance_to_neighbors {
                isolation_values.push(isolation);
            }
        }
        if isolation_values.is_empty() {
            println!("no isolation values available");
            return;
        }
        isolation_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        //get pollution stats
        let mut pollution_values: Vec<f64> = pollution_data.values().copied().collect();
        
        if pollution_values.is_empty() {
            println!("no pollution data available");
            return;
        }
        pollution_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        //set thresholds at 75th percentile - reasonable value that is not too restrictive or inclusive
        let p75_index = pollution_values.len() * 3 / 4;
        let pollution_threshold = pollution_values[p75_index];
        
        let i75_index = isolation_values.len() * 3 / 4;
        let isolation_threshold = isolation_values[i75_index];
        
        println!("using thresholds: pollution > {:.2}, isolation > {:.2} km", 
                 pollution_threshold, isolation_threshold);
        
        //find potiential gaps
        let gaps = self.find_monitoring_gaps(pollution_data, isolation_threshold, pollution_threshold);
        
        println!("Found {} stations in areas with monitoring gaps:", gaps.len());
        for (i, (station, pollution)) in gaps.iter().take(10).enumerate() {
            println!("  {}. {} ({}, {}): Pollution: {:.2}, Isolation: {:.2} km", 
                     i+1, 
                     station.site_name,
                     station.city_name,
                     station.state_name,
                     pollution,
                     station.avg_distance_to_neighbors.unwrap());
        }
        
        if gaps.len() > 10 {
            println!("  ... and {} more", gaps.len() - 10);
        }
    }
}