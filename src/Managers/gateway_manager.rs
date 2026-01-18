use anyhow::anyhow;
use crate::Models::Settings::settings_model::AppSettings;
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use crate::Models::message_model::ResponseMessage;
use crate::Models::report_service::reporte::Reporte;

pub fn validate_endpoint_code(code: &String, settings: AppSettings) -> anyhow::Result<(String, Endpoint)> {
    match code.as_str(){
        "01" => Ok((settings.report_service_url, Endpoint::Report)),
        "02" => Ok((settings.add_report_service_url, Endpoint::SendReport)),
        "03" => Ok((settings.registro_service_url, Endpoint::Registro)),
        "04" => Ok((settings.login_service_url, Endpoint::Login)),
        "05" => Ok((settings.route_service_url, Endpoint::Route)),
        "06" => Ok((settings.routes_service_url, Endpoint::Routes)),
        _=> Err(anyhow!("Invalid code {}", code))
    }
}

pub async fn send_request(endpoint: Endpoint, mut url: String, request: String) -> (StatusCode, Json<ResponseMessage>){
    let response  = match endpoint{
        Endpoint::Report => {
            send_get_request(url).await
        },
        Endpoint::SendReport => {
            send_post_request(url, request).await
        },
        Endpoint::Registro => {
            send_post_request(url, request).await
        },
        Endpoint::Login => {
            send_post_request(url, request).await
        },
        Endpoint::Route => {
            let param = format!("/{request}");
            url.push_str(param.as_str());
            send_get_request(url).await
        },
        Endpoint::Routes => {
            send_get_request(url).await
        }
    };
    match response{
        Ok(response) => {
            let response_message = ResponseMessage{
                response_code: String::from("000"),
                response
            };
            (StatusCode::OK, Json(response_message))
        },
        Err(err) => {
            let response_message = ResponseMessage{
                response_code: String::from("111"),
                response: String::from("Error al enviar request")
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response_message))
        }
    }
}

async fn send_post_request(url: String, request: String) -> Result<String>{
    let client = reqwest::Client::new();

    let response = match client.post(&url)
        .header("Content-Type", "application/json")
        .body(request)
        .send().await {
        Ok(response) => response,
        Err(e) => return Err(anyhow!(e))
    };

    let response_string = match response.text().await {
        Ok(response_string) => response_string,
        Err(e) => return Err(anyhow!(e))
    };
    Ok(response_string)
}

async fn send_get_request(url: String) -> Result<String>{
    let client = reqwest::Client::new();

    let response = match client.get(&url)
        .send().await{
        Ok(response) => response,
        Err(e) => return Err(anyhow!(e))
    };
    let response_string = match response.text().await{
        Ok(text) => text,
        Err(e) => return Err(anyhow!(e))
    };
    Ok(response_string)
}
pub enum Endpoint{
    Report,
    SendReport,
    Registro,
    Login,
    Route,
    Routes,
}