use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use base64;

#[derive(Deserialize, Serialize)]
pub struct AutoCompleteRequest {
    pub query: String,
}

#[derive(Deserialize, Serialize)]
pub struct SerpRequest {
    pub currency: String,
    // Add other fields as needed
}

#[derive(Serialize)]
pub struct SerpResponse {
    status: String,
    message: String,
}

#[derive(Serialize)]
pub struct SerpHotelResponse {
    status: String,
    message: HotelMessage,
}

#[derive(Serialize)]
pub struct HotelMessage {
    hotelID: String,
    minPrice: f64,
    currency: String,
}

pub async fn get_auto_complete(
    Json(payload): Json<AutoCompleteRequest>,
) -> impl IntoResponse {
    if payload.query.len() < 4 {
        return (
            StatusCode::BAD_REQUEST,
            Json(SerpResponse {
                status: "error".to_string(),
                message: "Query should be at least 4 letters".to_string(),
            }),
        )
        .into_response();
    }

    let url = "";
    let auth = format!(
        "Basic {}",
        
    );
    let client = Client::new();

    match client
        .post(url)
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            let data: serde_json::Value = response.json().await.unwrap();
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "data": data["data"],
                })),
            )
            .into_response()
        }
        Ok(response) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SerpResponse {
                status: "error".to_string(),
                message: format!("HTTP error! Status: {}", response.status()),
            }),
        )
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SerpResponse {
                status: "error".to_string(),
                message: format!("Something went wrong: {}", error),
            }),
        )
        .into_response(),
    }
}

pub async fn get_serp_hotels(
    Json(payload): Json<SerpRequest>,
) -> impl IntoResponse {
    let url = "";
    let auth = format!(
        "Basic {}",
        
    );
    let client = Client::new();

    match client
        .post(url)
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            let data: serde_json::Value = response.json().await.unwrap();

            if let Some(hotels) = data["data"]["hotels"].as_array() {
                if !hotels.is_empty() {
                    let hotel = &hotels[0];
                    let min_price = apply_profit_rate(
                        hotel["rates"][0]["daily_prices"][0].as_f64().unwrap(),
                    );
                    (
                        StatusCode::OK,
                        Json(SerpHotelResponse {
                            status: "ok".to_string(),
                            message: HotelMessage {
                                hotelID: hotel["id"].as_str().unwrap().to_string(),
                                minPrice: min_price,
                                currency: payload.currency.clone(),
                            },
                        }),
                    )
                    .into_response()
                } else {
                    (
                        StatusCode::OK,
                        Json(SerpResponse {
                            status: "unavailable".to_string(),
                            message: "Time unavailable.".to_string(),
                        }),
                    )
                    .into_response()
                }
            } else {
                (
                    StatusCode::OK,
                    Json(SerpResponse {
                        status: "unavailable".to_string(),
                        message: "No hotels available.".to_string(),
                    }),
                )
                .into_response()
            }
        }
        Ok(response) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SerpResponse {
                status: "error".to_string(),
                message: format!("HTTP error! Status: {}", response.status()),
            }),
        )
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SerpResponse {
                status: "error".to_string(),
                message: format!("Something went wrong: {}", error),
            }),
        )
        .into_response(),
    }
}

pub async fn get_serp_region(
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let url = "";
    let auth = format!(
        "Basic {}",
        
    );
    let client = Client::new();

    match client
        .post(url)
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            let data: serde_json::Value = response.json().await.unwrap();

            let hotel_ids: Vec<String> = data["data"]["hotels"]
                .as_array()
                .unwrap()
                .iter()
                .map(|hotel| hotel["id"].as_str().unwrap().to_string())
                .collect();

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "ok",
                    "message": hotel_ids,
                })),
            )
            .into_response()
        }
        Ok(response) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SerpResponse {
                status: "error".to_string(),
                message: format!("HTTP error! Status: {}", response.status()),
            }),
        )
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SerpResponse {
                status: "error".to_string(),
                message: format!("Something went wrong: {}", error),
            }),
        )
        .into_response(),
    }
}

fn apply_profit_rate(price: f64) -> f64 {
    price * 1.2 // Apply some profit rate logic here
}
