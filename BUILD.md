# Organizador POSTECH - Electron v1

## Arquitetura

```
┌─────────────────────────────────────────────────┐
│                  Electron (main.js)              │
│  ┌─────────────┐  ┌──────────────┐              │
│  │  Renderer    │  │  Main Process │              │
│  │  (index.html)│◄─┤  (IPC)        │              │
│  └──────┬───────┘  └──────┬───────┘              │
│         │                 │                      │
│         │    IPC          │  stdin/stdout (JSON)  │
│         │                 │                      │
│  ┌──────▼─────────────────▼───────┐              │
│  │     Rust Sidecar (backend)      │              │
│  │     - listar_videos             │              │
│  │     - agrupar_por_aula          │              │
│  │     - organizar_arquivos        │              │
│  │     - reverter_organizacao      │              │
│  │     - gerar_csv                 │              │
│  │     - salvar/ler_config         │              │
│  └─────────────────────────────────┘              │
└─────────────────────────────────────────────────┘
```

## Pré-requisitos

- **Node.js** 18+ (https://nodejs.org)
- **Rust** 1.70+ (https://rustup.rs)
- **Windows:** Visual Studio Build Tools ou MSVC
- **macOS:** Xcode Command Line Tools

## Desenvolvimento

```bash
# 1. Instalar dependências do Electron
npm install

# 2. Compilar o backend Rust
cd rust-backend
cargo build --release
cd ..

# 3. Rodar em modo desenvolvimento
npm start
```

## Build para Distribuição

### Windows (instalador NSIS)
```bash
npm run build:win
```
Saída: `dist/Organizador POSTECH_1.0.0_x64-setup.exe`

### macOS (DMG)
```bash
npm run build:mac
```
Saída: `dist/Organizador POSTECH-1.0.0-x64.dmg`

### Ambos
```bash
npm run build:all
```

## Estrutura do Projeto

```
Organizador_POSTECH_Electron_v1/
├── main.js              # Processo principal do Electron
├── preload.js           # Bridge segura (contextBridge)
├── package.json         # Dependências e config do electron-builder
├── src/
│   └── index.html       # Frontend (HTML/CSS/JS)
├── rust-backend/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Backend Rust (sidecar)
├── assets/
│   └── icon.ico         # Ícone do app
└── dist/                # Builds gerados
```

## Comunicação Electron ↔ Rust

O Electron inicia o Rust como processo filho (sidecar).
A comunicação é via stdin/stdout usando JSON lines:

**Electron → Rust (comando):**
```json
{"cmd": "listar_videos", "id": 12345, "caminho": "C:/Videos"}
```

**Rust → Electron (resposta):**
```json
{"type": "success", "id": 12345, "data": [...]}
{"type": "error", "id": 12345, "message": "Erro: ..."}
```

## Problemas Conhecidos e Soluções

### 1. "No native build was found" (ffi-napi)
**Não usamos ffi-napi.** A comunicação é via stdin/stdout, sem FFI.

### 2. Backend Rust não encontrado em produção
O `electron-builder` empacota o binário Rust como `extraResources`.
Caminho em produção: `resources/backend/organizador-postech-backend`

### 3. Build no macOS falha com "code signing"
Para distribuição fora da Mac App Store, usa:
```json
"mac": {
  "identity": null,
  "hardenedRuntime": false
}
```

### 4. Antivírus bloqueia o .exe
O binário Rust é compilado sem assinatura digital.
Para ambientes corporativos:
- Solicitar ao TI para adicionar exceção
- Ou assinar o binário com certificado (~$70/ano)

### 5. node_modules no frontendDist
O Tauri reclamava disso. O Electron não tem essa limitação,
mas é boa prática manter `node_modules` fora de `src/`.
