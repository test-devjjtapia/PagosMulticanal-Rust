# Pagos Multicanal Core (Rust)

Este proyecto es la evolución del "Sistema de Gestión de Pagos Multicanal" migrado de COBOL a Rust. Proporciona un orquestador de transacciones financieras altamente seguro y eficiente.

## Características

- **Arquitectura Modular**: Despacho de transacciones basado en canales (POS, WEB, MOVIL).
- **Seguridad por Diseño**: Uso del sistema de tipos de Rust para garantizar la integridad de los datos.
- **Asincronía**: (En proceso) Motor basado en `tokio` para alta concurrencia.
- **Validación de Idempotencia**: Evita duplicidad transaccional.

## Estructura del Proyecto

- `src/main.rs`: Orquestador y servidor API.
- `src/types.rs`: Modelos de datos y contratos.
- `src/business.rs`: Reglas de negocio y persistencia.
- `src/security.rs`: Cifrado y tokenización.

## Requisitos

- Rust 1.70+
- Cargo

## Ejecución

### Pruebas
```bash
cargo test
```

### Ejecutar Servidor
```bash
cargo run
```

## Estado de la Migración
El core ha sido migrado exitosamente desde el repositorio COBOL original, manteniendo compatibilidad con los contratos de datos establecidos.
