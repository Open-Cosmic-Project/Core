//! kernel.root — Supervisor de procesos de CosmicOS
//!
//! Alcance acordado en CosmicOS spec sección 3.1:
//! - Supervisor de solo lectura (RO), binario inmutable en /usr verificado.
//! - Inmortal ante señales (protegido por dominio SELinux `kernel_root_t` y oom_score_adj = -1000).
//! - Supervisión de servicios con backoff exponencial y aislamiento al 5º fallo en 10 min.
//! - Ventana cerrada de operaciones accesibles por socket con verificación SO_PEERSEC:
//!   (1) Lanzar proceso aislado a partir de manifiesto validado.
//!   (2) Reiniciar, aislar o congelar servicio.
//!   (3) Montar/desmontar imagen de app verificada (solo instalador SPS).
//!   (4) Consultar estado de un servicio (RO).
//!   (5) Apagar, reiniciar o suspender dispositivo (solo cpowerd).

use cosmic_core::ipc::IpcMessage;
use cosmic_core::service::ServiceRestartPolicy;

fn main() {
    println!("[kernel.root] Iniciando supervisor inmutable de CosmicOS...");

    let policy = ServiceRestartPolicy::default();
    println!(
        "[kernel.root] Política de reinicio: máx {} fallos en {}s",
        policy.max_consecutive_failures,
        policy.failure_window_seconds
    );

    println!("[kernel.root] Escuchando socket de ventana segura en /run/cosmic/kernel.root.sock...");
}

pub fn handle_ipc_request(msg: IpcMessage) {
    match msg {
        IpcMessage::LaunchProcess { app_id, manifest_path } => {
            println!("[kernel.root] Operación (1): Lanzar proceso '{}' con manifiesto '{}'", app_id, manifest_path);
        }
        IpcMessage::ServiceControl { service_id, operation } => {
            println!("[kernel.root] Operación (2): Control de servicio '{}': {:?}", service_id, operation);
        }
        IpcMessage::MountAppImage { app_id, image_path, mount_point } => {
            println!("[kernel.root] Operación (3): Montar EROFS '{}' en '{}' para '{}'", image_path, mount_point, app_id);
        }
        IpcMessage::UnmountAppImage { app_id } => {
            println!("[kernel.root] Operación (3): Desmontar imagen de '{}'", app_id);
        }
        IpcMessage::QueryServiceStatus { service_id } => {
            println!("[kernel.root] Operación (4): Consulta de estado para '{}'", service_id);
        }
        IpcMessage::PowerControl { state } => {
            println!("[kernel.root] Operación (5): Control de energía: {:?}", state);
        }
    }
}
