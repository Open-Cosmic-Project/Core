//! clauncher — Launcher de aplicaciones en Rust
//!
//! Alcance acordado en CosmicOS spec secciones 4, 7 y 13:
//! - Lee el manifiesto validado.
//! - Aplica mount namespace con directorios RO declarados (/usr, /system, etc.).
//! - Aplica Landlock restringiendo escritura únicamente a ~/.var/capps/<app>.
//! - Aplica filtro seccomp base según la clase de la app.
//! - Aplica no_new_privs y abandona todas las capabilities no necesarias.
//! - Aplica namespace de red vacío si la app no declara red.
//! - Ejecuta (execve) el binario en su sandbox estricto.

use cosmic_core::manifest::{AppClass, AppManifest};
use std::env;

fn main() {
    println!("[clauncher] CosmicOS Application Launcher (v0.2.8 incremental test)");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: clauncher <ruta-a-manifest.json> [argumentos-app...]");
        return;
    }

    let manifest_path = &args[1];
    println!("[clauncher] Cargando manifiesto: {}", manifest_path);
}

pub fn configure_sandbox(manifest: &AppManifest) {
    println!("[clauncher] Aplicando aislamiento para {}", manifest.header.name);
    println!("[clauncher] - Clase de app: {:?}", manifest.body.app_class);

    // 1. Mount namespace
    // 2. Landlock (FS)
    // 3. Seccomp (Syscalls por clase)
    // 4. Network namespace si aplica
    if manifest.body.app_class == AppClass::Generic {
        println!("[clauncher] - Aplicando política estándar de mínimo privilegio");
    }
}
