use reqwest;
use serde::{Deserialize, Serialize};

// Structure represents summary data of individual days.
#[derive(Debug, Deserialize, Serialize)]
pub struct DailySummary {
    id: u32,
    date: String,
    productivity_pulse: u32,
    very_productive_percentage: f64,
    productive_percentage: f64,
    neutral_percentage: f64,
    distracting_percentage: f64,
    very_distracting_percentage: f64,
    all_productive_percentage: f64,
    all_distracting_percentage: f64,
    uncategorized_percentage: f64,
    business_percentage: f64,
    communication_and_scheduling_percentage: f64,
    social_networking_percentage: f64,
    design_and_composition_percentage: f64,
    entertainment_percentage: f64,
    news_percentage: f64,
    software_development_percentage: f64,
    reference_and_learning_percentage: f64,
    shopping_percentage: f64,
    utilities_percentage: f64,
    total_hours: f64,
    very_productive_hours: f64,
    productive_hours: f64,
    neutral_hours: f64,
    distracting_hours: f64,
    very_distracting_hours: f64,
    all_productive_hours: f64,
    all_distracting_hours: f64,
    uncategorized_hours: f64,
    business_hours: f64,
    communication_and_scheduling_hours: f64,
    social_networking_hours: f64,
    design_and_composition_hours: f64,
    entertainment_hours: f64,
    news_hours: f64,
    software_development_hours: f64,
    reference_and_learning_hours: f64,
    shopping_hours: f64,
    utilities_hours: f64,
    total_duration_formatted: String,
    very_productive_duration_formatted: String,
    productive_duration_formatted: String,
    neutral_duration_formatted: String,
    distracting_duration_formatted: String,
    very_distracting_duration_formatted: String,
    all_productive_duration_formatted: String,
    all_distracting_duration_formatted: String,
    uncategorized_duration_formatted: String,
    business_duration_formatted: String,
    communication_and_scheduling_duration_formatted: String,
    social_networking_duration_formatted: String,
    design_and_composition_duration_formatted: String,
    entertainment_duration_formatted: String,
    news_duration_formatted: String,
    software_development_duration_formatted: String,
    reference_and_learning_duration_formatted: String,
    shopping_duration_formatted: String,
    utilities_duration_formatted: String,
}

impl DailySummary {
    //TODO: Fix up code comment here
    // Fetch summary of time data logged during the previous two weeks from RescueTime daily_summary API. Does not include the current day.
    //TODO: Currently takes API Key as an argument. Okay for development, but need to eventually switch to Oauth2
    pub fn fetch(key: &String) -> Result<Vec<DailySummary>, reqwest::Error> {
        let request_url = format!(
            "https://www.rescuetime.com/anapi/daily_summary_feed?key={}",
            key
        );

        //TODO: Currently using blocking get method. Should switch to async eventually.
        // Fetch daily summary data from rescuetime api and store in Vector wrapped in Result type.
        let response = reqwest::blocking::get(&request_url)?.json::<Vec<DailySummary>>();

        response
    }
}
