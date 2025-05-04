mod station;
mod network;
mod data;
use std::error::Error;
use network::MonitoringNetwork;
fn main() -> Result<(), Box<dyn Error>> {
    //read the station data
    println!("Reading station data...");
    let stations = data::read_stations("aqs_sites.csv")?;
    println!("Loaded {} stations", stations.len());
    
    //create monitoring network
    let mut network = MonitoringNetwork::new();
    for station in stations {
        network.add_station(station);
    }
    
    //build adjacency list (calculate all distances)
    println!("Building adjacency list (calculating distances between stations)...");
    network.build_adjacency_list();
    println!("Built adjacency list");
    
    //calculate average distance to 10 nearest neighbors
    println!("Calculating isolation metrics (distance to 10 nearest neighbors)...");
    network.calculate_isolation(10);
    println!("Calculated isolation metrics");
    
    //print isolation statistics
    network.print_isolation_statistics();
    
    //read pollution data
    println!("Reading pollution data...");
    let pollution_data = data::read_pollution("annual_conc_by_monitor_2023.csv")?;
    println!("Loaded pollution data for {} stations", pollution_data.len());
    
    //analyze correlation
    let correlation = network.analyze_correlation(&pollution_data);
    println!("Correlation between isolation and pollution: {:.4}", correlation);
    
    //find stations with high pollution and high isolation
    println!("Finding monitoring gaps (high pollution, high isolation)...");
    network.find_and_print_monitoring_gaps(&pollution_data);

    println!("analysis complete");
    Ok(())
}
