use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistroRequest{
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub password: String,
}