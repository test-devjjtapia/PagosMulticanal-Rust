use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Channel {
    POS,
    WEB,
    MOVIL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    pub canal: Channel,
    pub cliente_id: String,
    pub trace_id: String,
    pub timestamp: String,
}

// Módulos de negocio migrados a business.rs

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionRequest {
    pub id: String,
    pub monto: f64,
    pub moneda: String,
    pub pan_token: String,
    pub metadata: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TransactionStatus {
    OK,
    ERR,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionResponse {
    pub id: String,
    pub estado: TransactionStatus,
    pub codigo: String,
    pub mensaje: String,
    pub procesador: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorInfo {
    pub codigo: String,
    pub mensaje: String,
}
