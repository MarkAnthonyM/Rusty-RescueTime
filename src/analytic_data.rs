use crate::parameters::Parameters;
use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Struct represents individual cell data related to the row_headers field of the AnalyticData struct.
//TODO: Evaulate other possible options of modeling structures for data deserialization
#[derive(Debug, Deserialize, Serialize)]
pub struct SizeFour<T, U> {
    pub perspective: T,
    pub time_spent: i32,
    pub number_of_people: i32,
    pub restrict_kind: U,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SizeSeven<T> {
    pub perspective: T,
    pub time_spent: i32,
    pub number_of_people: i32,
    pub activity: String,
    pub document: String,
    pub category: String,
    pub productivity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SizeSix<T> {
    pub perspective: T,
    pub time_spent: i32,
    pub number_of_people: i32,
    pub activity: String,
    pub category: String,
    pub productivity: i32,
}

//TODO: Current method of deserialization feels too messy. Try to find A more concise way to work with json data
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum QueryKind {
    SizeFourInt(SizeFour<i32, i32>),
    SizeFourMixedInt(SizeFour<i32, String>),
    SizeFourMixedString(SizeFour<String, i32>),
    SizeFourString(SizeFour<String, String>),
    SizeSevenInt(SizeSeven<i32>),
    SizeSevenString(SizeSeven<String>),
    SizeSixInt(SizeSix<i32>),
    // SizeSixString(SizeSix<String>),
    SizeSixString(SizeSix<NaiveDateTime>),
}

// Struct represents data fetched from RescueTime analytic data API endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticData {
    pub notes: String,
    pub row_headers: Vec<String>,
    pub rows: Vec<QueryKind>,
}

//TODO: Explore generic struct

impl AnalyticData {
    // Send request to RescueTime analytic data API endpoint, and return deserialized response.
    pub fn fetch(
        key: &String,
        param: Parameters,
        format: String,
    ) -> Result<AnalyticData, reqwest::Error> {
        let request_url = param.construct_query(key, format);

        let response = reqwest::blocking::get(&request_url)?.json::<AnalyticData>();

        response
    }
}
