use crate::parameters::Parameters;
use serde::{ Deserialize, Serialize };

// Struct represents individual cell data related to the row_headers field of the AnalyticData struct.
//TODO: Evaulate other possible options of modeling structures for data deserialization 
#[derive(Debug, Deserialize, Serialize)]
struct RowCell {
    rank: i32,
    time_spent: i32,
    number_of_people: i32,
    activity: String,
    category: String,
    productivity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Rank {
    rank: i32,
    time_spent: i32,
    number_of_people: i32,
    activity: String,
    category: String,
    productivity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Interval {
    date: String,
    time_spent: i32,
    number_of_people: i32,
    activity: String,
    category: String,
    productivity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct RestrictProductivity {
    rank: i32,
    time_spent: i32,
    number_of_people: i32,
    productivity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum QueryKind {
    Rank(Rank),
    Interval(Interval),
    Restrict(RestrictProductivity),
}

// Struct represents data fetched from RescueTime analytic data API endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticData {
    notes: String,
    row_headers: Vec<String>,
    rows: Vec<RowCell>,

}

//TODO: Explore generic struct

impl AnalyticData {
    // Send request to RescueTime analytic data API endpoint, and return deserialized response.
    pub fn fetch(key: &String, param: Parameters, format: String) -> Result<AnalyticData, reqwest::Error> {
        let request_url = param.construct_query(key, format);

        let response = reqwest::blocking::get(&request_url)?.json::<AnalyticData>();
        
        response
    }
}