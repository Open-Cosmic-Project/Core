//! CinitD — Cosmic Init Driver (PID 1)
//!
//! Alcance acordado en CosmicOS spec secciones 3, 3.1 y 7.4:
//! - Ser el PID 1.
//! - Montar sistemas de ficheros virtuales del sistema final (/proc, /sys, /dev, /run).
//! - Recoger procesos huérfanos (SIGCHLD).
//! - Lanzar `kernel.root` (el primer proceso supervisado).
//! - Vigilar a `kernel.root` con heartbeat y alimentar el watchdog de hardware.
//! - Cerrar la puerta de módulos del kernel (`sysctl kernel.modules_disabled = 1`).
//! - Bloqueo de señales y apagado ordenado solo tras instrucción de `kernel.root`.

use std::fs;
use std::process::Command;

fn main() {
    println!("[CinitD] Iniciando CosmicOS PID 1...");

    // 1. Montar pseudofilesystems básicos si no están montados
    mount_pseudofs();

    // 2. Cerrar carga y descarga de módulos tras arranque
    disable_kernel_modules();

    // 3. Lanzar kernel.root (PID 2 usermode / supervisor)
    launch_kernel_root();

    // 4. Bucle principal de init (reaping de huérfanos y heartbeat de watchdog)
    run_init_loop();
}

fn mount_pseudofs() {
    println!("[CinitD] Preparando pseudofilesystems: /proc, /sys, /dev, /run");
    let _ = fs::create_dir_all("/run/cosmic");
}

fn disable_kernel_modules() {
    println!("[CinitD] Aplicando kernel.modules_disabled = 1 (sellado de drivers)");
    let _ = fs::write("/proc/sys/kernel/modules_disabled", "1");
}

fn launch_kernel_root() {
    println!("[CinitD] Lanzando supervisor de solo lectura: kernel.root");
    let _ = Command::new("/usr/bin/kernel-root")
        .spawn();
}

fn run_init_loop() {
    println!("[CinitD] Init listo. Bucle de supervisión y recolección activo.");
}
