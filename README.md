Air Quality Monitoring Network Analysis

A. Project Overview
Goal
This project analyzes the effectiveness of the US air quality monitoring network by examining how monitoring stations are distributed geographically in relation to pollution levels. The key question is: "How well-positioned are US air quality monitoring stations to detect pollution hotspots?"
Dataset
Air Quality System (AQS) Sites: EPA monitoring station metadata including geographic coordinates (20,888 stations)
Annual Concentrations by Monitor 2023: PM2.5 pollution measurements (864 stations with valid data)
Both datasets are directly from the EPA's Air Quality System database

B. Data Processing
Loading
Used Serde for CSV deserialization with custom data structures
Handled renaming of CSV columns to match Rust struct fields
Implemented robust error handling to continue processing despite missing or invalid data
Cleaning & Transformations
Generated unique station IDs by combining state, county, and site codes
Filtered PM2.5 measurements to use only consistent "24-HR BLK AVG" sampling method
Prioritized annual average measurements for more stable pollution values
Handled missing latitude/longitude values by skipping invalid entries

C. Code Structure
Modules
station.rs: Defines station data structure and ID generation
network.rs: Implements graph representation and analysis algorithms
data.rs: Handles data loading and transformation
main.rs: Coordinates the overall analysis workflow
Key Functions & Types
Station Struct
Purpose: Represents a monitoring station with its metadata and calculated metrics
Fields: Geographic coordinates, location information, isolation metric
MonitoringNetwork Struct
Purpose: Represents the graph of monitoring stations
Components: HashMap of stations and adjacency list with distances
build_adjacency_list()
Purpose: Creates the graph structure with stations as nodes
Logic: Uses spatial indexing and distance filtering to optimize calculations
Output: Adjacency list with distances between stations
calculate_isolation()
Purpose: Determines how isolated each station is from others
Logic: Calculates average distance to k nearest neighbors
Output: Updates each station with isolation metric
analyze_correlation()
Purpose: Examines relationship between isolation and pollution
Logic: Calculates Pearson correlation coefficient
Output: Correlation value
find_monitoring_gaps()
Purpose: Identifies areas that may benefit from additional monitoring
Logic: Finds stations with both high pollution and high isolation
Output: List of stations meeting both criteria
Main Workflow
Load station data
Create monitoring network
Build adjacency list with distance-based connections
Calculate isolation metrics
Load pollution data
Analyze correlation between isolation and pollution
Identify monitoring gaps

D. Tests
Test Output
running 3 tests
test test_haversine_distance ... ok
test test_calculate_isolation ... ok
test test_correlation_calculation ... ok

Test Descriptions
test_haversine_distance: Verifies distance calculation accuracy using known geographic points
test_calculate_isolation: Ensures the isolation metric is properly calculated for each station
test_correlation_calculation: Tests the correlation analysis with controlled input data

E. Results
Program Output
Isolation statistics (km to 10 nearest neighbors):
  Minimum: 0.00 km
  Maximum: 162.17 km
  Median: 6.37 km
  Mean: 12.39 km
  
Correlation between isolation and pollution: -0.1985

using thresholds: pollution > 10.00, isolation > 16.88 km
Found 36 stations in areas with monitoring gaps:
  1. Southwest Minnesota Regional Airport (Marshall, Minnesota): Pollution: 10.76, Isolation: 68.54 km
  2. LOCATED ABOUT 1/4 MILE WEST OF SITE (Not in a City, Michigan): Pollution: 10.36, Isolation: 61.50 km
  3. LOSTWOOD NWR (Not in a City, North Dakota): Pollution: 10.12, Isolation: 46.56 km
  4. CHANUTE (Not in a City, Kansas): Pollution: 11.66, Isolation: 42.82 km
  5. Sandersville (Sandersville, Georgia): Pollution: 10.60, Isolation: 42.36 km
  6. Yreka (Yreka, California): Pollution: 10.95, Isolation: 42.27 km
  7. Gainesville (Gainesville, Georgia): Pollution: 10.43, Isolation: 40.98 km
  8. Colville-E 1St (Colville, Washington): Pollution: 10.20, Isolation: 39.58 km
  9. Oakridge, Willamette Center Trailer (OAK or WAC) (Oakridge, Oregon): Pollution: 10.21, Isolation: 38.16 km
  10. Hoopa Valley Reservation (Not in a City, California): Pollution: 15.68, Isolation: 38.12 km
  ... and 26 more

Interpretation
The average monitoring station has neighbors within 12.39 km, indicating fairly dense coverage
The slight negative correlation (-0.1985) suggests that less isolated areas tend to have slightly higher pollution levels
36 stations were identified as having both high pollution and high isolation (75th percentile thresholds)
The most isolated high-pollution station is in Minnesota, with an average of 68.54 km to its 10 nearest neighbors
These identified gaps could benefit from additional monitoring coverage

F. Usage Instructions
Building and Running

Build the project 
cargo build --release

Run the analysis
cargo run --release

Expected Runtime
Total runtime: ~5-10 seconds on a modern computer
The adjacency list calculation is the most time-intensive operation but has been optimized with spatial indexing

G. AI-Assistance Disclosure

Spatial Indexing Optimization: Received assistance from ChatGPT for implementing the grid-based spatial indexing approach to reduce distance calculation complexity- original program took 10+ minutes to run
I implemented this by dividing geographic space into 1-degree grid cells and only calculating distances between stations in adjacent cells
This reduced the computational complexity from O(n²) to approximately O(n) since each station only computes distances to a limited number of potential neighbors

Correlation Formula Implementation: Referenced ChatGPT for the Pearson correlation coefficient formula
I understood this as calculating the covariance between two variables (isolation and pollution) divided by the product of their standard deviations

To formulate this documentation into bullet points:
Asked ChatGPT to turn my word-vomit paragraph into documentation that matched the format provided on piazza
