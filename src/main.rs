pub mod types;
pub mod business;
pub mod security;

use ax_um::{
    routing::{post, get},
    Json, Router,
};
use std::net::SocketAddr;
use types::{Context, TransactionRequest, TransactionResponse, TransactionStatus};
use security::SecurityProcessor;

// Alias para facilitar la lectura del código con Axum
mod ax_um {
    pub use axum::*;
}

pub struct Orchestrator;

impl Orchestrator {
    pub async fn dispatch(ctx: Context, mut req: TransactionRequest) -> TransactionResponse {
        let security = SecurityProcessor::new();
        
        println!("Orchestrator: Despachando canal {:?} con Trace: {}", ctx.canal, ctx.trace_id);

        // Tokenización de PAN (Seguridad avanzada)
        if !req.pan_token.starts_with("TOK-") {
            req.pan_token = security.tokenize_pan(&req.pan_token);
        }

        let mut resp = TransactionResponse {
            id: req.id.clone(),
            estado: TransactionStatus::ERR,
            codigo: "".to_string(),
            mensaje: "".to_string(),
            procesador: "".to_string(),
        };

        // Lógica de negocio asíncrona
        if let Err(e) = business::idempotency::validate(&ctx, &req).await {
            resp.codigo = e.codigo;
            resp.mensaje = e.mensaje;
            return resp;
        }

        resp.procesador = business::processor::resolve(&ctx, &req).await;

        if let Err(e) = business::persistence::save(&ctx, &req, &mut resp).await {
            resp.codigo = e.codigo;
            resp.mensaje = e.mensaje;
        }

        resp
    }
}

// Handlers para la API REST
async fn pay_handler(Json(payload): Json<(Context, TransactionRequest)>) -> Json<TransactionResponse> {
    let (ctx, req) = payload;
    let response = Orchestrator::dispatch(ctx, req).await;
    Json(response)
}

async fn health_check() -> &'static str {
    "Pagos Multicanal Core is UP"
}

#[tokio::main]
async fn main() {
    println!("--- Pagos Multicanal Rust API Server ---");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/pay", post(pay_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Channel;

    #[tokio::test]
    async fn test_orchestrator_async_flow() {
        let ctx = Context {
            canal: Channel::POS,
            cliente_id: "TEST-001".to_string(),
            trace_id: "TRACE-TEST".to_string(),
            timestamp: "now".to_string(),
        };
        let req = TransactionRequest {
            id: "TRX-TEST".to_string(),
            monto: 10.0,
            moneda: "USD".to_string(),
            pan_token: "1234567812345678".to_string(), // PAN real para probar tokenización
            metadata: "Test".to_string(),
        };

        let resp = Orchestrator::dispatch(ctx, req).await;
        assert_eq!(resp.estado, TransactionStatus::OK);
        assert!(resp.procesador.contains("ASYNC"));
    }
}
