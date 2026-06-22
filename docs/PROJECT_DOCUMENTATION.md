# Organizador POSTECH - Electron v1

## Documentação Completa do Projeto

---

## 1. Visão Geral

### 1.1 O que é
Aplicação desktop para organização de arquivos de vídeo acadêmicos, desenvolvida para o curso POSTECH (FIAP). Organiza vídeos em pastas por aula, gera relatórios (TXT e HTML), cria planilhas e gerencia templates de email.

### 1.2 Tecnologias
- **Frontend**: HTML + CSS + JavaScript (Renderer Process do Electron)
- **Backend**: Rust (binário sidecar compilado)
- **Comunicação**: stdin/stdout JSON (Rust) ↔ IPC (Electron)

### 1.3 Estrutura de Pastas
```
src/
├── index.html          (UI principal - monolito HTML/CSS/JS)
├── styles/main.css     (estilos globais)
└── modules/            (módulos JS - futuro)
rust-backend/
├── src/main.rs         (lógica de filesystem, projetos, relatórios)
├── Cargo.toml
└── target/release/     (binário compilado)
docs/
└── PROJECT_DOCUMENTATION.md
```

---

## 2. Arquitetura

### 2.1 Fluxo de Comunicação
```
[Renderer JS] --IPC--> [Electron Main] --stdin--> [Rust Backend]
[Renderer JS] <--IPC-- [Electron Main] <--stdout-- [Rust Backend]
```

### 2.2 Comandos Rust (enum `Command`)
| Comando | Parâmetros | Descrição |
|---------|-----------|-----------|
| `organizarArquivos` | `caminho, modo, criarExtras` | Organiza vídeos em pastas |
| `reverterOrganizacao` | `caminho, deletarRelatorios` | Desfaz organização |
| `listarVideos` | `caminho` | Lista vídeos em uma pasta |
| `agruparPorAula` | `videos` | Agrupa vídeos por padrão "Aula N" |
| `salvarRelatorio` | `caminho, conteudo` | Salva relatório .txt |
| `salvarRelatorioHtml` | `caminho, conteudo` | Salva relatório .html |
| `salvarProjeto` | `nome, dados` | Salva estado do projeto |
| `carregarProjeto` | `nome` | Carrega estado do projeto |
| `listarProjetos` | - | Lista projetos salvos |
| `deletarProjeto` | `nome` | Deleta um projeto |
| `renomearProjeto` | `nome_antigo, nome_novo` | Renomeia um projeto |
| `listarArquivos` | `caminho, padrao` | Lista arquivos por padrão |
| `salvarConfig` | `config` | Salva configurações |
| `lerConfig` | - | Lê configurações |
| `gerarCsv` | `caminho, grupos, titulos` | Gera planilha CSV |
| `copiarParaClipboard` | `texto` | Copia texto |

---

## 3. Funcionalidades

### 3.1 Organizar Arquivos (aba única)
- **Modo Automático**: Detecta padrões "Aula N" nos nomes dos vídeos e cria pastas automaticamente
- **Modo Manual**: Permite definir número de pastas e opções extras (Capítulo de Projeto, Onboarding)
- **Modo Reverso**: Move vídeos de volta para a raiz e remove pastas vazias
- **Opções de organização** (expansíveis, só para Manual): qtd pastas, pastas extras, sem relatório
- **Opções de desorganização** (expansíveis, só para Reverso): deletar relatórios

### 3.2 Editor de Emails
- Gera assunto e corpo automaticamente baseado no caminho da pasta
- Dropdown "Tipo de vídeo" altera formatação (Matéria, Boas Vindas, Onboarding, Extra)
- Botões de copiar assunto e corpo
- Campos de caminho editáveis (Entrada, Sigla, Fase, Matéria) com botões de copiar

### 3.3 Gerador de Planilhas
- Gera tabela HTML com links para vídeos
- Nomes limpos (sem extensão, sem versão)
- Botão de copiar como HTML para Google Sheets
- Gera arquivo CSV

### 3.4 Relatórios
- **TXT**: Relatório texto simples com estrutura e durações
- **HTML** (opcional): Relatório visual bonito com:
  - Cores do tema POSTECH Escuro
  - Estrutura de pastas expansível
  - Duração por aula com botões de copiar e highlight persistente
  - Lista de vídeos por aula com botões de copiar caminho
  - Resumo com estatísticas

### 3.5 Sistema de Projetos
- Salvar/carregar projetos (caminho, campos de email)
- AutoSave com prefixo do projeto + data/hora
- Mantém até 5 slots por projeto (deleta o mais antigo)
- Renomear e deletar projetos
- Auto-carregar último projeto na inicialização

### 3.6 Perfis de Cores
- POSTECH Escuro (padrão)
- POSTECH Claro
- Neon Hacker
- Pink Sakura
- Industrial
- Carbon
- Criar, renomear, duplicar e excluir perfis customizados
- Suporte a gradientes e imagem de fundo

### 3.7 Atalhos de Teclado
- 1-9 para navegar entre painéis (sem Ctrl/Cmd)
- Ctrl+Enter para executar automático
- Ctrl+R para reverso
- Ctrl+S para planilha
- Ctrl+P para salvar projeto
- Editáveis com até 3 teclas combinadas (dropdowns para modificadores)
- Adaptativo Mac/Windows (⌘ no Mac, Ctrl no Windows)

### 3.8 Backup/Restauração
- Exporta JSON com configurações, atalhos, perfis e projetos
- Importa e restaura tudo

### 3.9 Versões
- Busca atualizações no GitHub
- Auto-check ao iniciar (configurável)
- Backup antes de atualizar

### 3.10 Tutorial Guiado
- 10 passos interativos com balões posicionados
- Roda na primeira execução
- Pode ser revisitado em Extras > Tutorial Guiado

---

## 4. Configurações

### 4.1 Seções Expansíveis
- **Comportamento**: Confirmação antes de executar
- **Aparência**: Fonte (tamanho, peso, estilo), predefinições
- **Perfis de Cores**: 22 campos de cor, gradiente, imagem de fundo
- **Salvamento Automático**: Desligado, 1, 5, 10, 15, 20 min
- **Atalhos de Teclado**: Editor com dropdowns
- **Relatório**: Opção de gerar HTML
- **Versões**: Auto-check, backup antes de atualizar

### 4.2 Persistência
- `localStorage`: configurações, atalhos, perfis de cores
- Sistema de arquivos (Rust): projetos salvos como JSON

---

## 5. Problemas Conhecidos e Soluções

### 5.1 Email bold
- **Problema**: `font-weight` global sobrescrevia negrito do email
- **Solução**: CSS específico com `!important` e `<style>` inline no painel

### 5.2 Relatório lento
- **Problema**: Rust lia header de cada vídeo (parser MP4/MKV)
- **Solução**: Extração de duração via JS com `<video preload="metadata">`

### 5.3 Auto-carregamento
- **Problema**: `onchange` não dispara ao definir valor programaticamente
- **Solução**: Chamada explícita de `extractPathInfo()` após carregar

### 5.4 Atalhos ativando com modificadores
- **Problema**: Atalho `Shift+1` ativava só com `Shift`
- **Solução**: Ignorar teclas modificadoras sozinhas no listener

---

## 6. Manutenção

### 6.1 Adicionar novo comando Rust
1. Adicionar variante no enum `Command` em `main.rs`
2. Adicionar handler no `match` do `main()`
3. Implementar função de lógica
4. Adicionar chamada no JavaScript via `invokeRust()`

### 6.2 Adicionar novo painel
1. Adicionar `<div class="panel" id="panel-xxx">` no HTML
2. Adicionar item na sidebar com `data-panel="panel-xxx"`
3. Adicionar entrada no objeto `titles` no JavaScript
4. Adicionar atalho de teclado (opcional)

### 6.3 Atualizar CSS
- Cores principais: variáveis CSS em `:root`
- Tema claro: `[data-theme="light"]`
- Perfis customizados: `applyColorProfile()`

---

*Documentação gerada em: 2026-06-18*
*Versão do projeto: Electron v1*
*Última atualização: 2026-06-18*
