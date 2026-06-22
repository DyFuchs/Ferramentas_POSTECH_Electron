# Análise Cross-Platform - Organizador POSTECH Electron v1

## Objetivo
Garantir que a aplicação funcione corretamente em Windows, macOS e Linux.

## Análise Detalhada

### 1. Sistema de Arquivos

#### 1.1 Separadores de Caminho
- **Windows**: `\` (barra invertida)
- **macOS/Linux**: `/` (barra)
- **Solução**: Usar `path.join()` do Node.js no main process. No Rust, usar `std::path::Path::join()` que é cross-platform.
- **Status**: ✅ Já implementado

#### 1.2 Caminho do Binário Rust
- **Windows**: `rust-backend/target/release/organizador-postech-backend.exe`
- **macOS/Linux**: `rust-backend/target/release/organizador-postech-backend`
- **Solução**: O `main.js` já adiciona `.exe` no Windows (`if (process.platform === 'win32')`)
- **Status**: ✅ Já implementado

#### 1.3 Diretório de Dados da Aplicação
- **Windows**: `%APPDATA%/organizador-postech/`
- **macOS**: `~/Library/Application Support/organizador-postech/`
- **Linux**: `~/.local/share/organizador-postech/`
- **Solução**: Usar `dirs::data_dir()` do Rust (crate `dirs`) que é cross-platform
- **Status**: ✅ Já implementado

#### 1.4 Permissões de Arquivo
- **Windows**: Geralmente sem problemas em pastas do usuário
- **macOS**: Pode precisar de permissões em pastas protegidas
- **Linux**: Depende das permissões do usuário
- **Solução**: Sempre usar pastas do usuário, nunca pastas do sistema
- **Status**: ✅ Já implementado

### 2. Clipboard

#### 2.1 Windows
- **Solução atual**: `clip.exe` com stdin piped
- **Problemas**: `clip.exe` pode não estar em PATH em algumas versões
- **Alternativa**: `powershell Set-Clipboard` (mais lento mas sempre disponível)
- **Melhor solução**: Usar `clipboard.writeText()` do Electron via IPC (cross-platform)
- **Status**: ✅ Implementado

#### 2.2 macOS
- **Solução**: `pbcopy` (sempre disponível)
- **Status**: ✅ Implementado

#### 2.3 Linux
- **Solução**: `xclip` ou `wl-copy`
- **Problema**: Nem sempre instalado
- **Melhor solução**: Usar `clipboard.writeText()` do Electron via IPC
- **Status**: ✅ Implementado

### 3. Duração de Vídeos

#### 3.1 Problema
- `ffprobe` não está instalado por padrão em nenhuma plataforma
- Parser MP4 puro em Rust é lento para arquivos grandes
- Solução atual usa `<video preload="metadata">` no navegador (Electron)

#### 3.2 Solução Recomendada
- **Melhor opção**: Usar `MediaInfo.js` (WebAssembly) no renderer process
  - Cross-platform (roda no navegador)
  - Suporta todos os formatos de vídeo
  - Muito rápido (compilado para WASM)
  - Não requer instalação externa
- **Alternativa**: Usa `mp4` crate no Rust para MP4/MKV, e estimativa por tamanho para outros formatos
- **Status**: ⚠️ Implementado com `<video preload="metadata">` (pode ser lento para muitos vídeos)

### 4. Atalhos de Teclado

#### 4.1 Problema
- Atalhos podem conflitar com atalhos do sistema operacional
- **Windows**: `Ctrl+Alt+...` pode conflitar com atalhos do sistema
- **macOS**: `Cmd+...` é o padrão, `Ctrl+...` pode conflitar
- **Linux**: Varia conforme o ambiente desktop

#### 4.2 Solução
- Usar `Cmd/Ctrl+Shift+...` para evitar conflitos
- No macOS, preferir `Cmd` em vez de `Ctrl`
- Permitir edição dos atalhos pelo usuário
- Usar `electron.globalShortcut` para atalhos globais (funcionam mesmo sem foco)
- **Status**: ❌ Ainda não implementado

### 5. Salvamento Automático

#### 5.1 Problema
- Precisa persistir dados entre inicializações
- Precisa ser cross-platform
- Precisa não bloquear a UI

#### 5.2 Solução
- Usar arquivo JSON no diretório de dados da aplicação
- Usar `setInterval` no renderer process para salvamento periódico
- Salvar estado a cada mudança significativa (debounce de 2s)
- **Status**: ❌ Ainda não implementado

### 6. Sistema de Projetos

#### 6.1 Problema
- Precisa salvar/carregar estado completo da aplicação
- Precisa interface para gerenciar projetos
- Precisa nomenclatura automática

#### 6.2 Solução
- Cada projeto = arquivo JSON no diretório de dados
- Interface com lista de projetos, botões para salvar/carregar/deletar
- Nomenclatura automática: `{curso} - {sigla} - {fase} - {matéria} - {data}`
- **Status**: ❌ Ainda não implementado

### 7. Janelas e Diálogos

#### 7.1 Diálogos de Arquivo
- **Solução**: `dialog.showOpenDialog()` do Electron (cross-platform)
- **Status**: ✅ Já implementado

#### 7.2 Modal de Confirmação
- **Solução**: HTML/CSS inline (não depende de bibliotecas externas)
- **Status**: ✅ Já implementado

### 8. Build e Distribuição

#### 8.1 Windows
- **Instalador**: NSIS (já configurado)
- **Build**: `electron-builder --win --x64`
- **Status**: ✅ Configurado

#### 8.2 macOS
- **Instalador**: DMG (já configurado)
- **Build**: `electron-builder --mac --x64`
- **Problema**: Precisa de Mac para compilar
- **Solução**: Usar CI/CD (GitHub Actions) para build automático
- **Status**: ⚠️ Configurado mas não testado

#### 8.3 Linux
- **Instalador**: AppImage ou DEB
- **Build**: `electron-builder --linux --x64`
- **Status**: ❌ Não configurado

## Problemas Conhecidos e Correções Necessárias

### Críticos
1. **Duração de vídeos**: Solução atual pode ser lenta para muitos vídeos
   - **Correção**: Usar WebAssembly (MediaInfo.js) ou WebCodecs API

2. **Caminho do binário Rust**: O binário compilado no WSL não funciona no Windows
   - **Correção**: Sempre compilar no Windows para gerar o .exe

### Moderados
3. **Atalhos de teclado**: Ainda não implementados
   - **Correção**: Implementar sistema de atalhos editável

4. **Salvamento automático**: Ainda não implementado
   - **Correção**: Implementar com setInterval e debounce

5. **Sistema de projetos**: Ainda não implementado
   - **Correção**: Implementar interface e persistência

### Menores
6. **CSS do modal**: O CSS do modal está fora do `</html>` (adicionado com `cat >>`)
   - **Correção**: Mover para dentro do `<style>` principal

7. **Email com negrito**: Pode não estar funcionando em todas as plataformas
   - **Correção**: Usar `style="font-weight:700"` inline (já implementado)

## Conclusão
A aplicação é majoritariamente cross-platform. Os principais pontos de atenção são:
1. Compilar o Rust na plataforma correta (Windows para .exe, macOS para binário Mac)
2. Usar APIs do Electron para funcionalidades nativas (clipboard, diálogos)
3. Usar `<video preload="metadata">` para duração de vídeos (cross-platform)
4. Testar em macOS para garantir que atalhos e permissões funcionem
