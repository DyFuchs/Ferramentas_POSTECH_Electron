# PROTOCOLO DE ATUALIZAÇÃO DE VERSÃO — Organizador POSTECH

## Versão Atual: 0.1.3

---

## 1. PONTOS DE ATENDEÇÃO (Checklist de Versão)

### 1.1 Build & Package
- [ ] `package.json` → campo `"version": "X.Y.Z"`
- [ ] `main.js` → usa `app.getVersion()` (automático, sem edição manual)

### 1.2 Frontend — src/index.html
- [ ] **Declaração `APP_VERSION`** (linha ~952, segundo `<script>`):
  ```js
  const APP_VERSION = 'X.Y.Z';
  ```
- [ ] **Sidebar** (linha ~18):
  ```html
  <span id="sidebar-version">X.Y.Z</span>
  ```
- [ ] **Credits** (linha ~333):
  ```html
  <span id="credits-version">X.Y.Z</span>
  ```
  > NOTA: Estes dois spans são atualizados automaticamente via JS:
  ```js
  document.querySelectorAll('#sidebar-version, #credits-version').forEach(el => { el.textContent = APP_VERSION; });
  ```
  > Mas o HTML base também deve estar correto para o primeiro render (antes do JS executar).

### 1.3 Backend Rust
- [ ] `rust-backend/src/main.rs` → não possui referência de versão (OK)
- [ ] `rust-backend/Cargo.toml` → opcional, não usado pelo updater

### 1.4 GitHub Actions Workflow
- [ ] `.github/workflows/build.yml` → usa `package.json` version para nomear assets
- [ ] Assets de release devem seguir o padrão: `Ferramentas POSTECH-X.Y.Z-setup.exe`

---

## 2. PROCEDIMENTO DE RELEASE

### Passo 1: Atualizar os arquivos
1. Editar `package.json` → `"version": "X.Y.Z"`
2. Editar `src/index.html` → `const APP_VERSION = 'X.Y.Z';`
3. Editar `src/index.html` → `<span id="sidebar-version">X.Y.Z</span>`
4. Editar `src/index.html` → `<span id="credits-version">X.Y.Z</span>`

### Passo 2: Commit e Tag
```powershell
cd "H:\_______HERMES\hermes-desktop-docker\config\users\Dy_Fuchs\Projetos\Organziador de Arquivos POSTECH\Organizador_POSTECH_Electron_v1"
git add -A
git commit -m "vX.Y.Z - descrição das mudanças"
git tag vX.Y.Z
git push origin main
git push origin vX.Y.Z
```

### Passo 3: Verificar GitHub Actions
- Acessar: https://github.com/DyFuchs/Ferramentas_POSTECH_Electron/actions
- Aguardar a build completar
- Verificar se os assets foram gerados com o nome correto (vX.Y.Z)

### Passo 4: Publicar Release
- Acessar: https://github.com/DyFuchs/Ferramentas_POSTECH_Electron/releases
- Editar a release gerada automaticamente
- Adicionar notas de release
- **Publicar** (não deixar como Draft)

### Passo 5: Testar Updater
- Instalar a versão anterior no Windows host
- Abrir o app e verificar se detecta a nova versão
- Testar o download e instalação

---

## 3. REGRAS CRÍTICAS

1. **NUNCA deletar e recriar a mesma tag** — isso causa cache inconsistente no GitHub
2. **Sempre incremente a versão** — nunca reutilize uma versão anterior
3. **Sincronize `package.json` e `APP_VERSION`** — devem ser sempre idênticos
4. **Verifique os assets da release** — o nome do arquivo deve conter a versão correta
5. **Release deve ser "Published"** — não "Draft", pois o electron-updater não detecta drafts

---

## 4. ESTRUTURA DE VERSÃO

| Componente | Onde é lido | Usado por |
|------------|-------------|-----------|
| `package.json` `"version"` | electron-builder | electron-updater (compara com GitHub releases) |
| `APP_VERSION` (JS) | src/index.html | UI, fallback de update manual |
| `sidebar-version` (HTML) | src/index.html | Sidebar (primeiro render) |
| `credits-version` (HTML) | src/index.html | Créditos (primeiro render) |

---

## 5. HISTÓRICO DE VERSÕES

| Versão | Data | Mudanças |
|--------|------|----------|
| 0.1.0 | - | Versão inicial |
| 0.1.1 | - | - |
| 0.1.2 | - | Última versão estável antes dos fixes |
| 0.1.3 | 2026-06-25 | Fix lerRelatorio backend, report multi-format, interactive table, updater fix |
