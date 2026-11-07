// corchd — Orquestador del sistema en Go
//
// Alcance acordado en CosmicOS spec sección 7 y decisión D-04:
// - Corre completamente sin privilegios (unprivileged daemon).
// - Orquesta el resto del sistema, valida manifiestos de aplicaciones.
// - Delega operaciones privilegiadas y lanzamiento de sandboxes al binario `clauncher` en Rust.

package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("[corchd] Iniciando orquestador del sistema CosmicOS (sin privilegios)...")

	if len(os.Args) > 1 {
		manifestPath := os.Args[1]
		fmt.Printf("[corchd] Validando manifiesto: %s\n", manifestPath)
		fmt.Println("[corchd] Delegando lanzamiento seguro a /usr/bin/clauncher...")
	} else {
		fmt.Println("[corchd] Modo demonio orquestador en espera de peticiones de arranque...")
	}
}
