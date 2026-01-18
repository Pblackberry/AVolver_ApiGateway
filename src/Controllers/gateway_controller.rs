use axum::Json;

use crate::Models::message_model::{RequestMessage, ResponseMessage};
use crate::Models::Settings::settings_model;
use anyhow::{anyhow, Result};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use crate::Managers::gateway_manager::{send_request, validate_endpoint_code};
use crate::Models::Settings::settings_model::AppSettings;


pub async fn redirect_petition(State(state): axum::extract::State<Arc<AppSettings>>,
                               Json(payload): Json<RequestMessage>)
    -> (StatusCode, Json<ResponseMessage>)
{
    let settings = AppSettings{
        report_service_url: state.report_service_url.clone(),
        add_report_service_url: state.add_report_service_url.clone(),
        registro_service_url: state.registro_service_url.clone(),
        login_service_url: state.login_service_url.clone(),
        route_service_url: state.route_service_url.clone(),
        routes_service_url: state.routes_service_url.clone(),
    };

    let endpoint = match validate_endpoint_code(&payload.endpoint_id, settings){
        Ok(endpoint) => endpoint,
        Err(_)=> {
            let response = ResponseMessage{
                response_code: String::from("222"),
                response: String::from("No existe un endpoint con el codigo asignado")
            };
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    send_request(endpoint.1, endpoint.0, payload.request).await
}



