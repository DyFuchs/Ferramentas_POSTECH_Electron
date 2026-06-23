# CODEMAP - Ferramentas POSTECH Electron
# Mapa completo do codigo para navegacao eficiente
# Atualizado: 2026-06-23

## ESTRUTURA DE ARQUIVOS

src/
  index.html              # HTML principal + JS inline (~3071 linhas)
  styles.css              # CSS extraído do <style> inline
  modules/
    api.js                # Comunicacao com Rust via IPC
    utils.js              # Utilitarios gerais (escapeHtml, fmtDuration, etc.)
    ui-helpers.js         # Toast, modais, confirm
  rust-backend/
    src/main.rs           # Backend Rust (~798 linhas)

## ORDEM DE CARREGAMENTO

1. <link rel="stylesheet" href="styles.css">
2. <script src="modules/api.js"></script>
3. <script src="modules/utils.js"></script>
4. <script src="modules/ui-helpers.js"></script>
5. <script> ... inline ... </script>

## FUNCOES DE CONFIGURACAO

### buildConfig() (~linha 540)
Lê TODOS os valores da UI e retorna objeto para salvar:
- last_used_path, confirm_required, theme
- font_size, font_weight, font_style
- shortcuts, activeProfile, colorProfiles (camelCase - formato JS)
- auto_save_minutos (do select cfg-autosave)
- report_html (do checkbox cfg-report-html)
- auto_check_updates, backup_before_update (do localStorage)

### saveConfig() (~linha 588)
- Chama buildConfig() para obter valores da UI
- Envia ao Rust via invokeRust('salvarConfig', { config })
- Salva no localStorage (chave: organizador_config) SEMPRE (mesmo quando Rust succeed)
- Feedback visual: muda texto do botão + toast

### loadConfig() (~linha 619)
- Tenta ler do Rust via invokeRust('lerConfig')
- Verifica se resposta é objeto valido (nao string/array) - protecao contra race condition
- Fallback: lê do localStorage (organizador_config)
- Aplica valores na UI com suporte a AMBOS os formatos:
  - confirm_required / confirmRequired (verifica snake_case primeiro, depois camelCase)
  - colorProfiles / color_profiles
  - activeProfile / active_profile
- Aplica checkboxes de versão (via querySelector no versions-editor)
- Aplica checkbox de relatório (cfg-report-html)

### saveConfigSilent(cfg) (~linha 570)
- Limpa campos null/undefined do objeto
- Envia ao Rust via invokeRust('salvarConfig', { config })
- Sem feedback visual

### setupAutoSave() (~linha 1886)
- SÍNCRONO (sem await)
- Usa buildConfig() diretamente (nao loadConfig())
- Lê auto_save_minutos do buildConfig()
- Configura setInterval com minutos * 60 * 1000
- Atualiza select na UI

### updateAutoSaveSetting() (~linha 1904)
- Chamada onchange do select cfg-autosave
- Lê valor do select, chama buildConfig(), seta auto_save_minutos
- Chama saveConfigSilent(cfg) para salvar
- Chama setupAutoSave() para aplicar
- Toast de feedback

### DOMContentLoaded (~linha 2580)
- Carrega configuração salva (await loadConfig())
- Aplica colorProfiles com suporte a ambos formatos: savedCfg.colorProfiles || savedCfg.color_profiles
- Aplica activeProfile com suporte a ambos: savedCfg.activeProfile || savedCfg.active_profile
- Aplica perfil de cores via applyColorProfile()
- Carrega atalhos do localStorage
- Inicializa auto-save via setupAutoSave()

## RUST BACKEND (ConfigApp)

### ConfigApp struct (~linha 25 do main.rs)
- Usa #[serde(alias)] em CADA campo para aceitar ambos os formatos (camelCase e snake_case)
- NÃO usa rename_all (removido para evitar conflito)
- Campos com alias:
  - last_used_path / lastUsedPath
  - confirm_required / confirmRequired
  - auto_extra / autoExtra
  - fixed_dest / fixedDest
  - theme / theme
  - font_size / fontSize
  - font_weight / fontWeight
  - font_style / fontStyle
  - shortcuts / shortcuts
  - auto_save_minutos / autoSaveMinutos
  - active_profile / activeProfile
  - color_profiles / colorProfiles

### salvar_config() (~linha 568)
- Serializa ConfigApp para JSON e escreve em arquivo
- Retorna "Config salva" como string de sucesso

### ler_config() (~linha 577)
- Lê arquivo JSON e deserializa para ConfigApp
- Se arquivo não existe, retorna ConfigApp::default()
- Default: confirm_required=Some(true), theme=Some("dark"), font_size=Some("14"), etc.

### get_config_path() (~linha 649)
- Retorna %APPDATA%/organizador-postech/config.json

## FEEDBACK / GITHUB ISSUES

### Painel panel-feedback (~linha 267)
- Card adicionado no panel-extras com icone 📨
- Formulario com: tipo (bug/feature/feedback), titulo, descricao
- Info do sistema preenchida automaticamente (versao, plataforma, SO, Electron, Node)
- Botao "Enviar Feedback" abre GitHub Issues em nova aba com URL pre-preenchida
- Botao "Cancelar" volta para panel-extras

### Funcoes JavaScript
- `getSystemInfo()` — coleta versao, plataforma, SO, Electron, Node
- `submitFeedback()` — valida campos, monta URL do GitHub, abre no navegador
- `initFeedbackForm()` — atualiza div de info do sistema ao abrir painel
- Chamada via `showPanel('panel-feedback')` com `setTimeout(initFeedbackForm, 100)`

### URL do GitHub
- Formato: `https://github.com/GITHUB_REPO/issues/new?title=&body=&labels=`
- Labels: bug (para bug), enhancement (para feature), feedback (para geral)
- Prefixo do titulo: [BUG], [FEATURE] ou [FEEDBACK]

## AUTO-UPDATE

### Configuracao
- electron-updater no renderer (src/index.js)
- Handlers IPC no main.js: check-for-updates, download-update, quit-and-install
- Eventos: update-available, update-downloaded, update-error
- Verificacao automatica a cada 30min + ao iniciar (5s depois)

### Workflow de Release (.github/workflows/release.yml)
- 3 jobs paralelos: build-win, build-mac, build-linux
- Todos com --publish always (gera latest.yml, latest-mac.yml, latest-linux.yml)
- Job upload-release: baixa artifacts e adiciona a release
- draft: false (release publicada imediatamente)
- GH_TOKEN: ${{ github.token }} (token automatico do GitHub Actions)

### extraResources por plataforma
- Windows: rust-backend/target/release/organizador-postech-backend.exe -> backend/
- Mac: rust-backend/target/release/organizador-postech-backend -> backend/
- Linux: rust-backend/target/release/organizador-postech-backend -> backend/

## PROBLEMAS CONHECIDOS E SOLUCOES

### 1. invokeRust race condition
- Problema: handler nao faz matching de ID (Rust sempre envia id:0.0)
- Solucao: setupAutoSave usa buildConfig() diretamente (sincrono)
- Solucao: loadConfig verifica se resposta é objeto valido (nao string)

### 2. Conflito de nomes de campos (camelCase vs snake_case)
- JS usa snake_case (confirm_required, font_size, auto_save_minutos)
- Rust usava rename_all = "camelCase" (retirado)
- Solucao: #[serde(alias)] em cada campo do ConfigApp para aceitar ambos
- JS corrigido para verificar ambos os formatos no loadConfig e DOMContentLoaded

### 3. saveConfig nao salvava no localStorage quando Rust succeed
- saveConfig sava no localStorage apenas no catch (erro)
- Corrigido: salva SEMPRE no localStorage, independente do Rust

### 4. saveConfig simplificada duplicada
- Havia funcao saveConfig(cfg) simplificada na linha ~1978
- Removida para evitar conflito com saveConfig() completa

### 5. Checkboxes de versão renderizados dinamicamente
- renderVersionsEditor() gera HTML inline sem IDs fixos
- loadConfig() usa querySelector para encontrar checkboxes
- renderVersionsEditor() tambem le config do localStorage

### 6. APP_VERSION is not defined
- Era consequencia de erro de sintaxe anterior impedir o parse do const APP_VERSION
- Resolvido ao corrigir a sintaxe

### 7. extraResources glob nao funcionava no Mac
- Problema: glob "organizador-postech-backend*" nao copiava corretamente
- Solucao: extraResources separado por plataforma (win/mac/linux) com nomes explicitos
- Mac/Linux: binario sem .exe, Windows: binario com .exe

### 8. Mac icon nao encontrado
- Problema: assets/icon.icns nunca existia
- Solucao: Mac usa assets/icon.png (256x256) em vez de .icns

### 9. Linux AppImage nao gerava
- Problema: faltam dependencias (libfuse2, fakeroot)
- Solucao: step "Install Linux build dependencies" adicionado

### 10. GH_TOKEN nao chegava no Mac runner
- Problema: env do job nao propagava corretamente
- Solucao: GH_TOKEN passado via ${{ github.token }} (token automatico do GHA)

## SESSION LOG

- 2026-06-19: Modularizacao CSS + api.js + utils.js + ui-helpers.js
- 2026-06-19: Correcao de conflitos var/let entre modulos e inline
- 2026-06-19: Remocao de loadConfig simplificada duplicada
- 2026-06-19: Adicionado return config no loadConfig
- 2026-06-19: setupAutoSave revertido para sincrono (usa buildConfig)
- 2026-06-19: Removidos logs de debug
- 2026-06-19: Tentativa de matching de ID no invokeRust (revertida - Rust usa id:0.0 fixo)
- 2026-06-19: buildConfig expandido com report_html, auto_check_updates, backup_before_update
- 2026-06-19: loadConfig expandido para restaurar checkboxes de versão e relatório
- 2026-06-19: loadConfig verifica se resposta é objeto valido (race condition protection)
- 2026-06-19: Removida saveConfig(cfg) simplificada duplicada (conflito de nomes)
- 2026-06-19: saveConfig agora salva SEMPRE no localStorage
- 2026-06-19: Correcao de regex no extractPathInfo (barras invertidas duplicadas)
- 2026-06-19: Correcao de ConfigApp no Rust - removido rename_all, adicionado #[serde(alias)]
- 2026-06-19: loadConfig e DOMContentLoaded agora verificam ambos os formatos (snake_case + camelCase)
- 2026-06-19: Correcao de colorProfiles/activeProfile no DOMContentLoaded
- 2026-06-19: TODAS as configurações persistem corretamente ao reiniciar o app
- 2026-06-19: Adicionado painel de Feedback/GitHub Issues no Extras
- 2026-06-19: Funcoes getSystemInfo(), submitFeedback(), initFeedbackForm() implementadas
- 2026-06-19: Configurado electron-builder para publicacao no GitHub Releases
- 2026-06-19: Implementado auto-update com electron-updater (verificacao a cada 30min)
- 2026-06-19: Adicionados handlers IPC: check-for-updates, download-update, quit-and-install, open-url
- 2026-06-19: Adicionados listeners de eventos: update-available, update-downloaded, update-error
- 2026-06-19: package.json atualizado com scripts de build e release
- 2026-06-19: preload.js atualizado com APIs de update e openUrl
- 2026-06-19: GITHUB_REPO atualizado para DyFuchs/Ferramentas_POSTECH_Electron
- 2026-06-19: Titulo do painel de Backup simplificado para "8 • Backup"
- 2026-06-19: Titulo do painel de Feedback adicionado ao mapa de titulos
- 2026-06-19: Nome do app mudado de "Organizador" para "Ferramentas POSTECH" em todos os arquivos
- 2026-06-19: Versao alinhada para 0.1.0 em todo o código
- 2026-06-23: extraResources separado por plataforma (win/mac/linux) com nomes explicitos
- 2026-06-23: Mac Rust x86_64 para Intel Mac + cp para target/release/
- 2026-06-23: Mac icon redimensionado para 512x512
- 2026-06-23: Linux build dependencies adicionadas (libfuse2, fakeroot, rpm)
- 2026-06-23: Workflow com publish always em todas as plataformas
- 2026-06-23: Release draft: false
- 2026-06-23: sendToRust com retry (3 tentativas) para race condition
- 2026-06-23: Análise crítica: problemas identificados e corrigidos antes de executar
