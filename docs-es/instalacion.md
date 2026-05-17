# Instalacion

## Requisitos Previos

- Rust 1.70 o superior (instalado via [rustup](https://rustup.rs/))
- Una base de datos Money Manager EX (`.mmb`)

## Opcion 1: Instalar con Cargo

La forma mas sencilla de instalar `mmex_cli` es usando `cargo install`:

```bash
cargo install mmex_cli
```

Esto descargara el crate desde crates.io, lo compilara e instalara el binario `mmex` en tu directorio de binarios de Cargo (`~/.cargo/bin/`).

Asegurate de que `~/.cargo/bin` este en tu `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Verifica la instalacion:

```bash
mmex --version
```

## Opcion 2: Compilar desde el Codigo Fuente

Si prefieres compilar desde el codigo fuente:

```bash
# Clonar el repositorio
git clone https://github.com/PatricioRios/mmex-cli.git
cd mmex-cli

# Compilar en modo release
cargo build --release
```

El ejecutable se encontrara en `target/release/mmex`.

Puedes copiarlo a un directorio en tu `PATH`:

```bash
cp target/release/mmex /usr/local/bin/
```

O ejecutarlo directamente:

```bash
./target/release/mmex --version
```

## Autocompletado de Shell

`mmex_cli` puede generar scripts de autocompletado para tu shell. Una vez instalado, ejecuta:

```bash
# Bash
mmex completions bash > ~/.local/share/bash-completion/completions/mmex

# Zsh
mmex completions zsh > ~/.zfunc/_mmex

# Fish
mmex completions fish > ~/.config/fish/completions/mmex.fish

# PowerShell
mmex completions powershell > mmex.ps1
```

Despues de generar el script, reinicia tu shell o carga el archivo de completado segun corresponda.
