# PLANO DE MODULARIZACAO - Organizador POSTECH Electron
# Abordagem: Scripts <script src> (hibrida, sem bundler)
# Data: 2026-06-19
# Revisao: 3 (critica extrema)

## PRINCIPIOS FUNDAMENTAIS

1. Nunca remover funcao do inline ANTES de ter o modulo correspondente carregando via <script src>
2. Cada fase deve ser testavel independentemente
3. Backup funcional em: "Organizador_POSTECH_Electron_v1 - Copia (2)"
4. Ordem de carregamento segue dependencias
5. Cada modulo usa `var` para variaveis globais (scripts nao-module: var = window)
6. Cada modulo expoe funcoes via `window.fn = fn`
7. Cada modulo usa `if (typeof window !== 'undefined')` antes de acessar window

## REGRA DE OURO

EM CADA FASE:
1. Criar o arquivo do modulo
2. Adicionar <script src> no index.html ANTES do script inline
3. Testar se o app funciona
4. So entao remover a funcao do script inline
5. Testar novamente

NUNCA remover do inline ANTES de testar o modulo.

## ESTRUTURA ALVO

src/
  index.html                    # HTML limpo, so carrega <script src>
  app.js                        # Inicializacao (carregado por ultimo)
  modules/
    api.js                      # invokeRust, initAPI
    utils.js                    # escapeHtml, fmtDuration, cleanVideoName, isTextInputElement
    ui-helpers.js               # toast, handleError, safeExecute, modal
    config-manager.js           # buildConfig, saveConfig, loadConfig, saveConfigSilent
    theme-core.js               # applyColorProfile, defaultProfiles, colorProfiles, activeProfile
    theme-editor.js             # renderProfilesEditor, createNewProfile, deleteProfile, etc.
    shortcut-manager.js         # setupKeyboardShortcuts, handleKeyDown, saveShortcuts, etc.
    router.js                   # showPanel, toggleConfigSection
    path-extract.js             # extractPathInfo, togglePathExtract
    email.js                    # generateEmail, copySubject, copyBody
    excel.js                    # generatePlanilha, copyTable, downloadCSV
    report-core.js              # generateReportInBrowser, generateHtmlReport
    report-ui.js                # openReportFile, openReportFolder, copyHtmlPath, etc.
    projects.js                 # saveProject, refreshProjects, autoLoadLastProject
    versions.js                 # renderVersionsEditor, checkForUpdates
    backup.js                   # createBackup, restoreBackup
    autosave.js                 # setupAutoSave, updateAutoSaveSetting
    gradient-editor.js          # GradientEditor class

## ORDEM DE CARREGAMENTO NO index.html

<script src="modules/api.js"></script>            # 1. Comunicacao com Rust
<script src="modules/utils.js"></script>          # 2. Utilitarios gerais
<script src="modules/ui-helpers.js"></script>     # 3. Toast, modals, error handling
<script src="modules/config-manager.js"></script>  # 4. Configuracoes
<script src="modules/theme-core.js"></script>     # 5. Perfis de cores (core)
<script src="modules/shortcut-manager.js"></script> # 6. Atalhos de teclado
<script src="modules/router.js"></script>         # 7. Navegacao entre paineis
<script src="modules/path-extract.js"></script>   # 8. Extracao de caminho
<script src="modules/email.js"></script>          # 9. Editor de emails
<script src="modules/excel.js"></script>          # 10. Gerador de planilhas
<script src="modules/report-core.js"></script>    # 11. Relatorios (core)
<script src="modules/projects.js"></script>       # 12. Projetos
<script src="modules/versions.js"></script>       # 13. Versoes e atualizacoes
<script src="modules/backup.js"></script>         # 14. Backup/Restauracao
<script src="modules/autosave.js"></script>       # 15. Salvamento automatico
<script src="modules/theme-editor.js"></script>   # 16. Editor de perfis (UI)
<script src="modules/report-ui.js"></script>      # 17. Relatorios (UI)
<script src="modules/gradient-editor.js"></script> # 18. Editor de gradiente
<script src="app.js"></script>                    # 19. Inicializacao

## FASES DE EXECUCAO

### FASE 0: Preparacao
- Verificar que o backup e identico ao atual (md5)
- Criar pasta modules/ se nao existir
- Criar este plano em MODULARIZATION_PLAN.md

### FASE 1: api.js
Modulo: modules/api.js
Funcoes: initAPI, invokeRust, isElectronAvailable
Variaveis: var electronAvailable, var rustListeners
Dependencias: Nenhuma
No index.html:
  ADICIONAR: <script src="modules/api.js"></script> ANTES do script inline
  REMOVER do inline: initAPI(), invokeRust(), let electronAvailable, let rustListeners
Verificar: App abre, "Electron API disponivel" no console

### FASE 2: utils.js
Modulo: modules/utils.js
Funcoes: escapeHtml, fmtDuration, cleanVideoName, isTextInputElement
Dependencias: Nenhuma
No index.html:
  ADICIONAR: <script src="modules/utils.js"></script>
  REMOVER do inline: escapeHtml, fmtDuration, cleanVideoName, isTextInputElement
Verificar: App funciona, nomes de videos limpos, durations formatados

### FASE 3: ui-helpers.js
Modulo: modules/ui-helpers.js
Funcoes: toast, handleError, safeExecute, showConfirmModal, closeModal, confirmModal
Variaveis: var pendingAction
Dependencias: Nenhuma
No index.html:
  ADICIONAR: <script src="modules/ui-helpers.js"></script>
  REMOVER do inline: toast, handleError, safeExecute, showConfirmModal, closeModal, confirmModal, let pendingAction
Verificar: Toasts aparecem, modais funcionam, erros sao tratados

### FASE 4: config-manager.js
Modulo: modules/config-manager.js
Funcoes: buildConfig, saveConfig, loadConfig, saveConfigSilent
Variaveis: var activeProfile, var colorProfiles, var shortcuts, var electronAvailable
Dependencias: api.js
No index.html:
  ADICIONAR: <script src="modules/config-manager.js"></script>
  REMOVER do inline: buildConfig, saveConfig, loadConfig, saveConfigSilent, let activeProfile, let colorProfiles, let shortcuts
Verificar: Configuracoes salvas e restauradas, perfis de cores persistem

### FASE 5: theme-core.js
Modulo: modules/theme-core.js
Funcoes: applyColorProfile, openThemeQuickSwitch
Variaveis: var colorProfiles, var activeProfile
Dependencias: config-manager.js, ui-helpers.js
No index.html:
  ADICIONAR: <script src="modules/theme-core.js"></script>
  REMOVER do inline: applyColorProfile, openThemeQuickSwitch
Verificar: Perfis de cores funcionam, botao Perfil funciona

### FASE 6: shortcut-manager.js
Modulo: modules/shortcut-manager.js
Funcoes: setupKeyboardShortcuts, saveShortcuts, resetShortcuts, migrateShortcuts, keysMatch, executeShortcut, updateShortcutSelect, formatKeyCombo
Variaveis: var shortcuts, var MODIFIERS, var defaultShortcuts
Dependencias: ui-helpers.js, utils.js
No index.html:
  ADICIONAR: <script src="modules/shortcut-manager.js"></script>
  REMOVER do inline: setupKeyboardShortcuts, saveShortcuts, resetShortcuts, migrateShortcuts, keysMatch, executeShortcut, updateShortcutSelect, formatKeyCombo, let shortcuts, const MODIFIERS, const defaultShortcuts
Verificar: Atalhos funcionam, salvar/resetar atalhos funciona

### FASE 7: router.js
Modulo: modules/router.js
Funcoes: showPanel, toggleConfigSection, updateNavTooltips
Dependencias: ui-helpers.js
No index.html:
  ADICIONAR: <script src="modules/router.js"></script>
  REMOVER do inline: showPanel, toggleConfigSection, updateNavTooltips
Verificar: Navegacao entre paineis funciona

### FASE 8: path-extract.js
Modulo: modules/path-extract.js
Funcoes: extractPathInfo, togglePathExtract, copyField, copyText, updatePathStats, updatePathDurations
Dependencias: utils.js
No index.html:
  ADICIONAR: <script src="modules/path-extract.js"></script>
  REMOVER do inline: extractPathInfo, togglePathExtract, copyField, copyText, updatePathStats, updatePathDurations
Verificar: Extracao de caminho funciona, botoes de copiar funcionam

### FASE 9: email.js
Modulo: modules/email.js
Funcoes: generateEmail, copySubject, copyBody, copyToClipboard, fallbackCopyText, copyHtmlToClipboard
Dependencias: Nenhuma
No index.html:
  ADICIONAR: <script src="modules/email.js"></script>
  REMOVER do inline: generateEmail, copySubject, copyBody, copyToClipboard, fallbackCopyText, copyHtmlToClipboard
Verificar: Geracao de email funciona, copiar assunto/corpo funciona

### FASE 10: excel.js
Modulo: modules/excel.js
Funcoes: generatePlanilha, copyTable, downloadCSV, renderTable
Dependencias: Nenhuma
No index.html:
  ADICIONAR: <script src="modules/excel.js"></script>
  REMOVER do inline: generatePlanilha, copyTable, downloadCSV, renderTable
Verificar: Geracao de planilha funciona, copiar/CSV funciona

### FASE 11: report-core.js
Modulo: modules/report-core.js
Funcoes: generateReportInBrowser, generateHtmlReport
Dependencias: api.js, utils.js
No index.html:
  ADICIONAR: <script src="modules/report-core.js"></script>
  REMOVER do inline: generateReportInBrowser, generateHtmlReport
Verificar: Relatorio HTML funciona

### FASE 12: projects.js
Modulo: modules/projects.js
Funcoes: saveProject, refreshProjects, autoLoadLastProject, loadProject, deleteProject, editProjectName, saveEdit, getProjectAutoName
Variaveis: var projectMeta
Dependencias: api.js
No index.html:
  ADICIONAR: <script src="modules/projects.js"></script>
  REMOVER do inline: saveProject, refreshProjects, autoLoadLastProject, loadProject, deleteProject, editProjectName, saveEdit, getProjectAutoName, let projectMeta
Verificar: Salvar/carregar/deletar projetos funciona

### FASE 13: versions.js
Modulo: modules/versions.js
Funcoes: renderVersionsEditor, checkForUpdates, toggleAutoCheck, toggleBackupBeforeUpdate, startUpdate
Variaveis: const GITHUB_API_URL, const GITHUB_REPO
Dependencias: config-manager.js
No index.html:
  ADICIONAR: <script src="modules/versions.js"></script>
  REMOVER do inline: renderVersionsEditor, checkForUpdates, toggleAutoCheck, toggleBackupBeforeUpdate, startUpdate, const GITHUB_API_URL, const GITHUB_REPO
Verificar: Versoes aparecem, check funciona

### FASE 14: backup.js
Modulo: modules/backup.js
Funcoes: createBackup, restoreBackup
Dependencias: config-manager.js
No index.html:
  ADICIONAR: <script src="modules/backup.js"></script>
  REMOVER do inline: createBackup, restoreBackup
Verificar: Backup/Restauracao funciona

### FASE 15: autosave.js
Modulo: modules/autosave.js
Funcoes: setupAutoSave, updateAutoSaveSetting, autoSaveCurrentState, stopAutoSave
Variaveis: var autoSaveInterval, const AUTO_SAVE_TIMES
Dependencias: config-manager.js
No index.html:
  ADICIONAR: <script src="modules/autosave.js"></script>
  REMOVER do inline: setupAutoSave, updateAutoSaveSetting, autoSaveCurrentState, stopAutoSave, let autoSaveInterval, const AUTO_SAVE_TIMES
Verificar: Auto-save funciona, mudar minutagem funciona

### FASE 16: theme-editor.js
Modulo: modules/theme-editor.js
Funcoes: renderProfilesEditor, createNewProfile, deleteProfile, duplicateProfile, renameProfile, resetProfilesToDefault, updateProfileColor, updateProfileField, saveColorProfiles, toHexColor
Variaveis: var editingProfileName, const defaultProfiles, const profileColorFields
Dependencias: theme-core.js, ui-helpers.js
No index.html:
  ADICIONAR: <script src="modules/theme-editor.js"></script>
  REMOVER do inline: renderProfilesEditor, createNewProfile, deleteProfile, duplicateProfile, renameProfile, resetProfilesToDefault, updateProfileColor, updateProfileField, saveColorProfiles, toHexColor, let editingProfileName, const defaultProfiles, const profileColorFields
Verificar: Editor de perfis funciona, criar/deletar/renomear perfis funciona

### FASE 17: report-ui.js
Modulo: modules/report-ui.js
Funcoes: openReportFile, openReportFolder, copyHtmlPath, copyHtmlDur, copyAulaPath, toggleTree
Dependencias: report-core.js
No index.html:
  ADICIONAR: <script src="modules/report-ui.js"></script>
  REMOVER do inline: openReportFile, openReportFolder, copyHtmlPath, copyHtmlDur, copyAulaPath, toggleTree
Verificar: Botoes de copiar/abir relatorio funcionam

### FASE 18: gradient-editor.js
Modulo: modules/gradient-editor.js
Funcoes: GradientEditor (classe)
Dependencias: Nenhuma
No index.html:
  ADICIONAR: <script src="modules/gradient-editor.js"></script>
  REMOVER do inline: classe GradientEditor
Verificar: Editor de gradiente funciona

### FASE 19: app.js (inicializacao)
Modulo: src/app.js (raiz, NAO em modules/)
Funcoes: Inicializacao - DOMContentLoaded, initAPI, setupKeyboardShortcuts, setupAutoSave, setModoOrganizar, startTutorial, showTutorialStep, customConfirm, customPrompt, toggleTheme
Variaveis: const tutorialSteps, let tutorialCurrentStep
Dependencias: Todos os modulos anteriores
No index.html:
  ADICIONAR: <script src="app.js"></script> DEPOIS de todos os modulos
  REMOVER do inline: DOMContentLoaded handler, initAPI(), setupKeyboardShortcuts(), setupAutoSave(), setModoOrganizar(), startTutorial(), showTutorialStep(), customConfirm(), customPrompt(), toggleTheme(), const tutorialSteps, let tutorialCurrentStep
Verificar: App inicia corretamente, todas as funcoes inicializadas

### FASE 20: Limpeza Final
- Remover TODOS os blocos <script> inline do index.html
- Manter apenas os <script src="..."> na ordem correta
- Verificar que o HTML esta limpo (grep -c "function " no HTML deve ser 0)
- Testar no Windows completamente
- Verificar console do Electron para erros

## VERIFICACAO POR FASE

Cada fase deve passar nestes testes:
1. App abre sem erros no console do Electron
2. Funcoes da fase funcionam corretamente
3. Funcoes de fases anteriores continuam funcionando
4. Nenhum erro "is not defined" no console
5. Perfil de cores persiste ao reiniciar

## RISCOS E MITIGACOES

| Risco | Probabilidade | Mitigacao |
|-------|---------------|-----------|
| Funcao removida do inline antes do modulo carregar | ALTA | Regra de ouro: so remover DEPOIS de testar |
| Ordem de carregamento errada | MEDIA | Seguir a ordem de dependencias |
| var em modulo nao vai para window | BAIXA | Em scripts nao-module, var global = window |
| Modulo quebra funcoes de outros modulos | MEDIA | Testar cada fase independentemente |
| Erro de sintaxe no modulo | BAIXA | Verificar com node -e antes de adicionar |
| window.fn nao disponivel no onclick | BAIXA | Scripts carregam sequencialmente |

## CRONOGRAMA ESTIMADO

Fase 0: 10 min (preparacao)
Fases 1-18: ~15 min cada = ~270 min (4.5 horas)
Fase 19: 20 min (app.js)
Fase 20: 30 min (limpeza e testes)
Total: ~5.5 horas

## NOTAS TECNICAS IMPORTANTES

1. Em scripts <script src> (nao-module), `var x` cria `window.x` automaticamente
2. Em scripts <script src>, `let x` e `const x` NAO criam `window.x`
3. Por isso, nos modulos, usar `var` para variaveis que outros modulos precisam acessar
4. Funcoes declaradas com `function nome()` em scripts <script src> criam `window.nome` automaticamente
5. O app.js e o unico arquivo na raiz de src/ (nao em modules/)
6. A ordem de carregamento no index.html e CRITICA - nao mudar sem verificar dependencias
7. Se uma fase quebrar, reverter para o backup e investigar
