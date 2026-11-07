//! cdevd — Gestor de dispositivos (udev reducido)
//!
//! Alcance acordado en CosmicOS spec secciones 7.4 y 7.5:
//! - Gestiona nodos en /dev (creación, enlaces, permisos).
//! - Aplica reglas declarativas y etiquetado de dominios SELinux (cdev_t).
//! - Reacciona a eventos de uevent para periféricos soportados.
//! - NO carga módulos de kernel (bloqueado por SELinux y kernel.modules_disabled=1).

fn main() {
    println!("[cdevd] Iniciando gestor de dispositivos (/dev y reglas SELinux)...");
    println!("[cdevd] Escuchando eventos netlink kobject uevent del kernel...");
}
