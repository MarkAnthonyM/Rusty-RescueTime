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

// Struct represents data fetched from RescueTime analytic data API endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticData {
    notes: String,
    row_headers: Vec<String>,
    rows: Vec<RowCell>,

}

impl AnalyticData {
    // Send request to RescueTime analytic data API endpoint, and return deserialized response.
    pub fn fetch(key: &String) -> Result<AnalyticData, reqwest::Error> {
        let request_url = format!("https://www.rescuetime.com/anapi/data?key={}&perspective&format=json", key);

        //Todo: Implement query and format parameters and associated logic
        // let mut request_url = format!("https://www.rescuetime.com/anapi/data?key={}", key);
        
        // match queries {
        //     Some(queries) => {
        //         for query in queries {
        //             let str = format!("&{}", query);
        //             request_url.push_str(&str);
        //         }

        //         let data_format = format!("format={}", format);
        //         request_url.push_str(&data_format);
        //     },
        //     None => {
        //         let data_format = format!("format={}", format);
        //         request_url.push_str(&data_format);
        //     },
        // }

        let response = reqwest::blocking::get(&request_url)?.json::<AnalyticData>();

        response
    }
}