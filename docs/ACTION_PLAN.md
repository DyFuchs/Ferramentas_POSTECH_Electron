# PLANO DE AÇÃO DETALHADO — Organizador POSTECH
## Revisão e Refatoração Completa

---

## ÍNDICE
1. [Análise de Riscos e Limitações](#1-análise-de-riscos-e-limitações)
2. [Fase 1 — Limpeza e Segurança](#2-fase-1--limpeza-e-segurança)
3. [Fase 2 — Modularização JavaScript](#3-fase-2--modularização-javascript)
4. [Fase 3 — Qualidade e Testes](#4-fase-3--qualidade-e-testes)
5. [Fase 4 — Melhorias Contínuas](#5-fase-4--melhorias-contínuas)
6. [Estimativa de Esforço](#6-estimativa-de-esforço)

---

## 1. Análise de Riscos e Limitações

### 1.1 Riscos Identificados

| Risco | Probabilidade | Impacto | Mitigação |
|-------|--------------|---------|-----------|
| Quebrar funcionalidade existente durante refatoração | ALTA | ALTO | Fazer backup antes de cada fase; testar após cada mudança |
| Perda de dados do usuário (projetos salvos) | MÉDIA | CRÍTICO | Nunca modificar formato dos arquivos JSON; manter compatibilidade |
| Regressão de bugs já corrigidos | ALTA | MÉDIO | Criar checklist de funcionalidades críticas para verificar após cada fase |
| Incompatibilidade Electron após habilitar contextIsolation | MÉDIA | ALTO | Testar em ambiente de desenvolvimento antes de aplicar |
| Timeout no Rust durante operações longas | BAIXA | MÉDIO | Adicionar timeout nas chamadas invokeRust |
| localStorage quota excedida | BAIXA | BAIXA | Adicionar try/catch e aviso ao usuário |

### 1.2 Limitações Atuais

1. **Sem testes automatizados:** Qualquer mudança pode quebrar algo sem ser detectado
2. **Monolito JS:** 2.393 linhas em um arquivo dificulta mudanças isoladas
3. **Sem versionamento de dados:** Projetos salvos não têm versão do formato
4. **Sem backup automático antes de refatoração:** Risco de perda de dados
5. **Dependência de sessionStorage/localStorage:** Dados podem ser perdidos se o usuário limpar cache

### 1.3 Pré-requisitos Antes de Iniciar

1. **Backup completo** do projeto atual (commit git ou cópia de segurança)
2. **Checklist de funcionalidades críticas** para verificação após cada fase
3. **Ambiente de teste** separado do ambiente de produção

---

## 2. Fase 1 — Limpeza e Segurança

### 2.1 Remover Código Morto do Rust

**Arquivo:** `rust-backend/src/main.rs`

**Funções a remover:**
- `get_video_duration()` — 18 linhas (parsing lento de header MP4/MKV)
- `parse_mp4_duration()` — 28 linhas
- `parse_mkv_duration()` — 27 linhas
- `gerar_relatorio()` — 36 linhas (usa duração, agora gerada pelo JS)

**Risco:** Se alguma função JS ainda chamar `gerarRelatorio` no Rust, vai quebrar.

**Verificação necessária:**
```bash
grep -n "gerarRelatorio\|get_video_duration\|parse_mp4\|parse_mkv" src/index.html
```

**Solução:** Remover também o comando `GerarRelatorio` do enum `Command` e seu handler no match.

**Código a remover do Rust:**
```yaml
Remover do enum Command:
  - GerarRelatorio { caminho: String }

Remover do match:
  - Command::GerarRelatorio { caminho } => ...

Remover funções:
  - get_video_duration (linha ~157)
  - parse_mp4_duration (linha ~176)
  - parse_mkv_duration (linha ~205)
  - gerar_relatorio (linha ~630)

Remover imports não utilizados:
  - std::process::Command (se não usado em mais nada)
```

**Estimativa:** 30 minutos

---

### 2.2 Habilitar contextIsolation no Electron

**Arquivo:** `main.js`

**Mudança necessária:**
```javascript
// ANTES:
webPreferences: {
    nodeIntegration: false,
    sandbox: false,
    preload: path.join(__dirname, 'preload.js')
}

// DEPOIS:
webPreferences: {
    nodeIntegration: false,
    contextIsolation: true,  // ADICIONAR
    sandbox: true,           // MUDAR de false para true
    preload: path.join(__dirname, 'preload.js')
}
```

**Risco:** Alto. Pode quebrar a comunicação com o Rust se o preload.js não estiver correto.

**Mitigação:**
1. Verificar se o preload.js usa `contextBridge.exposeInMainWorld` corretamente
2. Testar todas as funcionalidades após a mudança
3. Se quebrar, reverter e investigar

**Verificação do preload.js:**
```javascript
// DEVE estar assim:
contextBridge.exposeInMainWorld('electronAPI', {
    // ... APIs expostas
});

// NÃO deve ter:
// window.minhaFuncao = ... (isso não funciona com contextIsolation)
```

**Estimativa:** 1 hora (incluindo testes)

---

### 2.3 Adicionar Content Security Policy

**Arquivo:** `src/index.html`

**Mudança:** Adicionar meta tag CSP no `<head>`:
```html
<meta http-equiv="Content-Security-Policy" content="
    default-src 'self';
    script-src 'self' 'unsafe-inline';
    style-src 'self' 'unsafe-inline';
    img-src 'self' data:;
    connect-src 'self';
">
```

**Risco:** Pode bloquear funcionalidades que dependem de recursos externos.

**Mitigação:** Usar `'unsafe-inline'` inicialmente e refinar depois.

**Estimativa:** 15 minutos

---

### 2.4 Adicionar Tratamento de Erros Centralizado

**Arquivo:** `src/index.html` (bloco `<script>`)

**Solução:** Criar função utilitária no início do JavaScript:
```javascript
// ============ ERROR HANDLING ============
function handleError(e, context) {
    console.error(`[${context}] Erro:`, e);
    const msg = e?.message || e?.toString() || 'Erro desconhecido';
    toast(`Erro em ${context}: ${msg}`, 'error');
}

function safeExecute(fn, context) {
    try {
        return fn();
    } catch (e) {
        handleError(e, context);
        return null;
    }
}

async function safeExecuteAsync(fn, context) {
    try {
        return await fn();
    } catch (e) {
        handleError(e, context);
        return null;
    }
}
```

**Aplicação:** Substituir todos os `catch(e) { console.log(...) }` por `catch(e) { handleError(e, 'nomeDaFuncao') }`.

**Estimativa:** 1 hora

---

### 2.5 Remover Referências a Painéis Antigos

**Problema:** Código ainda referencia `panel-auto`, `panel-manual`, `panel-reverse` que não existem mais.

**Verificação:**
```bash
grep -n "panel-auto\|panel-manual\|panel-reverse" src/index.html
```

**Correções necessárias:**
1. Remover cases `nav_auto`, `nav_manual`, `nav_reverse` do `executeShortcut`
2. Remover títulos antigos do objeto `titles`
3. Remover IDs de output antigos (`organize-output`, `manual-output`, `reverse-output`)

**Estimatia:** 30 minutos

---

## 3. Fase 2 — Modularização JavaScript

### 3.1 Estrutura de Módulos Proposta

```
src/
├── index.html                    (estrutura HTML limpa, < 300 linhas)
├── styles/
│   ├── main.css                  (estilos globais + variáveis CSS)
│   ├── components/
│   │   ├── sidebar.css
│   │   ├── panels.css
│   │   ├── forms.css
│   │   ├── buttons.css
│   │   ├── modals.css
│   │   └── tutorial.css
│   └── themes/
│       ├── postech-dark.css
│       ├── postech-light.css
│       └── custom-profiles.css
├── modules/
│   ├── app.js                    (inicialização, configuração global)
│   ├── router.js                 (navegação entre painéis)
│   ├── api.js                    (comunicação com Rust via IPC)
│   ├── ui-helpers.js             (toast, modals, loading, etc.)
│   ├── event-bindings.js         (todos os event listeners)
│   ├── report-manager.js         (relatórios TXT + HTML)
│   ├── tutorial-manager.js       (tutorial guiado)
│   ├── profile-manager.js        (perfis de cores)
│   ├── shortcut-manager.js       (atalhos de teclado)
│   ├── project-manager.js        (projetos + auto-save)
│   ├── email-manager.js          (editor de emails)
│   ├── excel-manager.js          (gerador de planilhas)
│   └── backup-manager.js         (backup/restauração)
└── index.html
    ├── <style> @import url('styles/main.css'); </style>
    ├── <script type="module" src="modules/app.js"></script>
    └── <div id="app">...</div>
```

### 3.2 Detalhamento de Cada Módulo

#### 3.2.1 `app.js` — Inicialização
```javascript
import { initRouter } from './router.js';
import { initAPI } from './api.js';
import { initEventBindings } from './event-bindings.js';
import { initTheme } from './profile-manager.js';
import { initShortcuts } from './shortcut-manager.js';
import { checkForUpdates } from './backup-manager.js';

async function initApp() {
    try {
        await initAPI();
        initTheme();
        initRouter();
        initEventBindings();
        initShortcuts();
        await autoLoadLastProject();
        checkForUpdates();
        console.log('[App] Inicialização completa');
    } catch (e) {
        handleError(e, 'initApp');
    }
}

document.addEventListener('DOMContentLoaded', initApp);
```

**Dependências:** router, api, event-bindings, profile-manager, shortcut-manager, backup-manager

**Risco:** Se a ordem de inicialização estiver errada, pode quebrar.

**Mitigação:** Usar `await` para dependências assíncronas.

---

#### 3.2.2 `api.js` — Comunicação com Rust
```javascript
let electronAvailable = false;

export async function initAPI() {
    if (window.electronAPI) {
        electronAvailable = true;
        window.electronAPI.onRustMessage(handleRustMessage);
    }
}

export function isElectronAvailable() {
    return electronAvailable;
}

export async function invokeRust(cmd, args = {}) {
    if (!electronAvailable) {
        throw new Error('Backend Rust não disponível');
    }
    const response = await window.electronAPI.sendCommand({ cmd, ...args });
    if (response.type === 'error') {
        throw new Error(response.data?.message || 'Erro no backend');
    }
    return response.data;
}

// Handlers para mensagens assíncronas do Rust
const rustListeners = [];
function handleRustMessage(msg) {
    rustListeners.forEach(fn => fn(msg));
}

export function onRustMessage(fn) {
    rustListeners.push(fn);
}
```

**Dependências:** Nenhuma (módulo base)

**Risco:** Mudar a API pode quebrar todos os outros módulos.

**Mitigação:** Manter a mesma assinatura de `invokeRust()`.

---

#### 3.2.3 `ui-helpers.js` — Utilitários de UI
```javascript
// ============ TOAST ============
export function toast(msg, type = 'info') {
    const container = document.getElementById('toast-container');
    if (!container) return;
    const el = document.createElement('div');
    el.className = 'toast ' + type;
    el.textContent = msg;
    container.appendChild(el);
    setTimeout(() => { if (el.parentNode) el.parentNode.removeChild(el); }, 3500);
}

// ============ MODAL ============
let modalAction = null;

export function showConfirmModal(title, message, details, action) {
    document.getElementById('modal-title').textContent = title;
    document.getElementById('modal-message').textContent = message;
    document.getElementById('modal-details').innerHTML = details || '';
    document.getElementById('confirm-modal').classList.add('active');
    modalAction = action;
}

export function closeModal() {
    document.getElementById('confirm-modal').classList.remove('active');
    modalAction = null;
}

export function confirmModal() {
    if (modalAction) modalAction();
    closeModal();
}

// ============ ERROR HANDLING ============
export function handleError(e, context) {
    console.error(`[${context}] Erro:`, e);
    const msg = e?.message || e?.toString() || 'Erro desconhecido';
    toast(`Erro: ${msg}`, 'error');
}

// ============ DOM HELPERS ============
export function $(id) {
    return document.getElementById(id);
}

export function $$(selector) {
    return document.querySelectorAll(selector);
}

export function isTextInputElement(el) {
    if (!el) return false;
    const tag = el.tagName.toLowerCase();
    return tag === 'input' || tag === 'textarea' || tag === 'select' || el.isContentEditable;
}
```

**Dependências:** Nenhuma (módulo base)

---

#### 3.2.4 `report-manager.js` — Relatórios
```javascript
import { invokeRust, $ } from './api.js';
import { toast, handleError } from './ui-helpers.js';

let lastReportPath = null;

export async function generateReport(path, grupos, outputElement) {
    outputElement.innerHTML = '<div class="loading"><div class="loading-spinner"></div> Calculando duracoes...</div>';
    
    try {
        const videos = await invokeRust('listarVideos', { caminho: path });
        const gruposComVideos = await invokeRust('agruparPorAula', { videos });
        
        // Calcula durações via JS (rápido)
        const durations = await calculateDurations(path, gruposComVideos);
        
        // Gera relatório TXO
        const txtReport = generateTxtReport(path, gruposComVideos, durations);
        await invokeRust('salvarRelatorio', { caminho: path, conteudo: txtReport });
        
        // Se opção HTML estiver ativa
        if ($('cfg-report-html')?.checked) {
            const htmlReport = generateHtmlReport(path, gruposComVideos, durations);
            await invokeRust('salvarRelatorioHtml', { caminho: path, conteudo: htmlReport });
        }
        
        lastReportPath = path;
        renderReportOutput(outputElement, path, gruposComVideos, durations);
        
    } catch (e) {
        handleError(e, 'generateReport');
    }
}

function generateHtmlReport(path, grupos, durations) {
    // ... (código atual de generateHtmlReport, agora como função do módulo)
}

function generateTxtReport(path, grupos, durations) {
    // ... (código de geração de relatório texto)
}

function renderReportOutput(element, path, grupos, durations) {
    // ... (código de renderização do output)
}

export function openReportFile() {
    if (lastReportPath && window.electronAPI?.openPath) {
        window.electronAPI.openPath(lastReportPath + '\\Relatorio_Organizacao.txt');
    }
}

export function openReportFolder() {
    if (lastReportPath && window.electronAPI?.openPath) {
        window.electronAPI.openPath(lastReportPath);
    }
}
```

**Dependências:** api, ui-helpers

**Risco:** Função `generateHtmlReport` tem 143 linhas. Deve ser dividida em sub-funções.

**Mitigação:**
```javascript
function generateHtmlReport(path, grupos, durations) {
    const styles = generateStyles();
    const header = generateHeader(path);
    const summary = generateSummary(grupos, durations);
    const durationTable = generateDurationTable(grupos, durations);
    const videoLists = generateVideoLists(path, grupos);
    const scripts = generateScripts();
    return `<!DOCTYPE html>...${styles}...${header}...${summary}...${durationTable}...${videoLists}...${scripts}</html>`;
}
```

---

#### 3.2.5 `shortcut-manager.js` — Atalhos de Teclado
```javascript
import { isTextInputElement, handleError } from './ui-helpers.js';
import { showPanel } from './router.js';
import { runAction } from './app.js';
import { saveProject } from './project-manager.js';
import { generateExcel } from './excel-manager.js';

const defaultShortcuts = {
    'nav_organizar': { keys: ['1'], label: 'Organizar Arquivos' },
    'nav_email':    { keys: ['2'], label: 'Editor de Emails' },
    // ... etc
};

let shortcuts = JSON.parse(JSON.stringify(defaultShortcuts));

export function initShortcuts() {
    // Carrega atalhos salvos
    const saved = localStorage.getItem('atalhos');
    if (saved) {
        try {
            const parsed = JSON.parse(saved);
            shortcuts = migrateShortcuts(parsed);
        } catch (e) {
            handleError(e, 'loadShortcuts');
        }
    }
    
    // Registra listener
    document.addEventListener('keydown', handleKeyDown);
}

function handleKeyDown(e) {
    // Ignora teclas modificadoras sozinhas
    if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) return;
    
    // Ignora em campos de texto (para atalhos sem modificadores)
    const hasMod = e.ctrlKey || e.shiftKey || e.altKey || e.metaKey;
    if (!hasMod && isTextInputElement(e.target)) return;
    
    for (const [action, sc] of Object.entries(shortcuts)) {
        if (keysMatch(e, sc.keys)) {
            e.preventDefault();
            e.stopPropagation();
            executeShortcut(action);
            return;
        }
    }
}

function keysMatch(event, keyCombo) {
    // ... (código atual de keysMatch)
}

function executeShortcut(action) {
    switch(action) {
        case 'nav_organizar': showPanel('panel-organizar'); break;
        case 'nav_email': showPanel('panel-email'); break;
        // ... etc
    }
}

export function saveShortcuts() {
    localStorage.setItem('atalhos', JSON.stringify(shortcuts));
    toast('Atalhos salvos!', 'success');
}

export function resetShortcuts() {
    shortcuts = JSON.parse(JSON.stringify(defaultShortcuts));
    saveShortcuts();
}
```

**Dependências:** ui-helpers, router, app, project-manager, excel-manager

**Risco:** Alto acoplamento com outros módulos.

**Mitigação:** Usar padrão Observer/Event Dispatcher para desacoplar.

---

### 3.3 Estratégia de Migração

**Abordagem:** Incremental (não reescrever tudo de uma vez)

**Passo 1:** Criar estrutura de pastas
```bash
mkdir -p src/styles/components src/styles/themes src/modules
```

**Passo 2:** Extrair CSS para arquivos separados
- Mover estilos globais para `src/styles/main.css`
- Mover estilos de componentes para `src/styles/components/`
- Adicionar `@import` no HTML

**Passo 3:** Extrair módulos JS um por um
1. `ui-helpers.js` (sem dependências)
2. `api.js` (depende de ui-helpers)
3. `router.js` (depende de ui-helpers)
4. `report-manager.js` (depende de api, ui-helpers)
5. ... etc

**Passo 4:** Atualizar HTML para usar módulos
```html
<script type="module">
    import { initApp } from './modules/app.js';
    initApp();
</script>
```

**Passo 5:** Remover código inline do HTML
- Substituir `onclick="..."` por `addEventListener`
- Substituir `style="..."` por classes CSS

---

## 4. Fase 3 — Qualidade e Testes

### 4.1 Configurar ESLint

**Arquivo:** `.eslintrc.json`
```json
{
    "env": {
        "browser": true,
        "es2022": true
    },
    "extends": "eslint:recommended",
    "rules": {
        "no-unused-vars": "warn",
        "no-undef": "error",
        "prefer-const": "warn",
        "no-var": "error",
        "semi": "error",
        "quotes": ["error", "single"],
        "indent": ["error", 4],
        "max-len": ["warn", 120],
        "complexity": ["warn", 10],
        "max-lines-per-function": ["warn", 50]
    }
}
```

### 4.2 Configurar Prettier

**Arquivo:** `.prettierrc`
```json
{
    "semi": true,
    "singleQuote": true,
    "tabWidth": 4,
    "printWidth": 120,
    "trailingComma": "none"
}
```

### 4.3 Testes Unitários (Jest)

**Estrutura:**
```
tests/
├── unit/
│   ├── shortcut-manager.test.js
│   ├── profile-manager.test.js
│   └── report-manager.test.js
└── integration/
    └── api.test.js
```

**Exemplo de teste:**
```javascript
// tests/unit/shortcut-manager.test.js
import { keysMatch } from '../../src/modules/shortcut-manager.js';

describe('keysMatch', () => {
    test('Shift+1 should not match Shift alone', () => {
        const event = { key: 'Shift', shiftKey: true, ctrlKey: false, altKey: false, metaKey: false };
        expect(keysMatch(event, ['Shift', '1'])).toBe(false);
    });
    
    test('Shift+1 should match when both pressed', () => {
        const event = { key: '1', shiftKey: true, ctrlKey: false, altKey: false, metaKey: false };
        expect(keysMatch(event, ['Shift', '1'])).toBe(true);
    });
});
```

---

## 5. Fase 4 — Melhorias Contínuas

### 5.1 Acessibilidade
- Adicionar atributos ARIA
- Suporte a navegação por teclado
- Contraste WCAG 2.1 AA

### 5.2 Internacionalização
- Extrair strings para arquivo de tradução
- Suportar pt-BR e en-US

### 5.3 Performance
- Cache de durações de vídeo
- Lazy loading de painéis
- Debounce em operações frequentes

### 5.4 CI/CD
- GitHub Actions para build automatizado
- Testes automatizados em cada PR
- Release automatizado com electron-builder

---

## 6. Estimativa de Esforço

| Fase | Tarefa | Estimativa | Risco |
|------|--------|------------|-------|
| 1.1 | Remover código morto do Rust | 30 min | Baixo |
| 1.2 | Habilitar contextIsolation | 1h | Alto |
| 1.3 | Adicionar CSP | 15 min | Baixo |
| 1.4 | Tratamento de erros centralizado | 1h | Médio |
| 1.5 | Remover referências antigas | 30 min | Baixo |
| **Fase 1 Total** | | **3h** | |
| 2.1 | Criar estrutura de pastas | 15 min | Baixo |
| 2.2 | Extrair CSS | 2h | Médio |
| 2.3 | Extrair módulos JS | 6h | Alto |
| 2.4 | Atualizar HTML | 2h | Alto |
| 2.5 | Testes de regressão | 2h | Médio |
| **Fase 2 Total** | | **12h** | |
| 3.1 | ESLint + Prettier | 1h | Baixo |
| 3.2 | Testes unitários | 4h | Médio |
| **Fase 3 Total** | | **5h** | |
| **TOTAL GERAL** | | **20h** | |

---

*Plano criado em: 2026-06-18*
*Baseado em: análise estática do código + pesquisa de melhores práticas 2025*
