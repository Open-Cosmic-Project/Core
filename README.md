# Core

Núcleo del espacio de usuario de CosmicOS.

| Componente | Qué hace |
|---|---|
| `cinitd` | CinitD, el init propio (PID 1), sin systemd |
| `kernel.root` | Supervisor de solo lectura: reinicia en su sandbox los procesos colgados |
| `clauncher` | Lanzador de apps: aplica la política del manifiesto y hace `exec` |
| `cdevd` | Demonio de dispositivos |
| `corchd` | Orquestador, escrito en Go |

`cinitd`, `kernel.root` y `clauncher` forman parte del hito 1.

## Compilar

```sh
cargo build --release -p cinitd
```

o, para todo el hito, `./cosmic-build milestone1` desde la raíz del árbol.

## Licencia

GPL-3.0-or-later.
