use dotenvy::dotenv;
use std::env;
pub struct AppSettings{
    pub report_service_url:String,
    pub add_report_service_url:String,
    pub registro_service_url:String,
    pub login_service_url:String,
    pub route_service_url:String,
    pub routes_service_url:String,
}

impl AppSettings{
    pub fn new_test_settings() ->AppSettings{
        dotenv().ok();
        AppSettings{
            report_service_url: env::var("REPORT_SERVICE_TEST").expect("Error al leer variables de entorno"),
            add_report_service_url: env::var("SEND_REPORT_SERVICE_TEST").expect("Error al leer variables de entorno"),
            registro_service_url: env::var("REGISTRO_SERVICE_TEST").expect("Error al leer variables de entorno"),
            login_service_url: env::var("LOGIN_SERVICE_TEST").expect("Error al leer variables de entorno"),
            route_service_url: env::var("ROUTE_SERVICE_TEST").expect("Error al leer variables de entorno"),
            routes_service_url: env::var("ROUTES_SERVICE_TEST").expect("Error al leer variables de entorno"),
        }
    }

    pub fn new_app_settings() ->AppSettings{
        if cfg!(debug_assertions){
            dotenv().ok();
        }
        AppSettings{
            report_service_url: env::var("REPORT_SERVICE").expect("Error al leer variables de entorno"),
            add_report_service_url: env::var("SEND_REPORT_SERVICE").expect("Error al leer variables de entorno"),
            registro_service_url: env::var("REGISTRO_SERVICE").expect("Error al leer variables de entorno"),
            login_service_url: env::var("LOGIN_SERVICE").expect("Error al leer variables de entorno"),
            route_service_url: env::var("ROUTE_SERVICE").expect("Error al leer variables de entorno"),
            routes_service_url: env::var("ROUTES_SERVICE").expect("Error al leer variables de entorno"),
        }
    }
}