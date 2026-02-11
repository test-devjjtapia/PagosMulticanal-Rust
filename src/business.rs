use crate::types::{Context, TransactionRequest, TransactionResponse, ErrorInfo, TransactionStatus};

pub mod idempotency {
    use super::*;
    
    pub async fn validate(_ctx: &Context, _req: &TransactionRequest) -> Result<(), ErrorInfo> {
        // Validación de duplicidad simulada
        Ok(())
    }
}

pub mod processor {
    use super::*;

    pub async fn resolve(_ctx: &Context, _req: &TransactionRequest) -> String {
        // Ruteo dinámico simulado
        "PROC-DEMO-RUST-ASYNC".to_string()
    }
}

pub mod persistence {
    use super::*;
    // En el futuro usaremos sqlx aquí
    // use sqlx::{Pool, Postgres};

    pub async fn save(_ctx: &Context, _req: &TransactionRequest, _resp: &mut TransactionResponse) -> Result<(), ErrorInfo> {
        println!("Persistence: Guardando transacción {} en DB", _req.id);
        _resp.estado = TransactionStatus::OK;
        Ok(())
    }
}
