use reqwest::{Client};
use serde_json::{Value};
use axum::{response::Json, http::StatusCode};

// Here Return as Result (ok, err) to handle the error for use of fetch_disaster_event_data call
pub async fn disaster_event_data() -> Result<Json<Value>, (StatusCode, String)> {

    // GDACS API endpoint URL
    let url = "https://www.gdacs.org/gdacsapi/api/events/geteventlist/EVENTS4APP";

    // Send GET request to GDACS API
    let client = Client::new();
    
    match client.get(url).send().await {
        Ok(response) => {
            // Parse the response as JSON
            match response.json::<Value>().await {
                Ok(json_data) => Ok(Json(json_data)),
                Err(e) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error parsing JSON: {}", e),
                )),
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching GDACS API: {}", e),
        )),
    }
}