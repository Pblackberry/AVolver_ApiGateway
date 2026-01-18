use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reporte {
    pub ruta_id: String,
    pub motivo: String,
    pub retrasoEstimado: i32
}