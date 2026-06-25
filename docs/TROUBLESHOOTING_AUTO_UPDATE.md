# PROBLEMAS E SOLUÇÕES — Auto-Update Mac/Linux

> Criado: 2026-06-25
> Status: AGUARDANDO REVISÃO DO USUÁRIO

---

## PROBLEMA 1: `Cannot find latest-mac.yml` (404)

### Descrição
O app no Mac installed (v0.1.2) tenta verificar atualizações ao iniciar. O `electron-updater` faz GET em:
```
https://github.com/DyFuchs/Ferramentas_POSTECH_Electron/releases/download/v0.1.2/latest-mac.yml
```
Retorna 404. O mesmo acontece para `latest.yml`.

### Diagnóstico
- A release v0.1.2 no GitHub tem 3 assets: `Ferramentas POSTECH-0.1.0.AppImage`, `Ferramentas POSTECH-0.1.0.dmg`, `Ferramentas POSTECH.Setup 0.1.0.exe`
- **Nenhum `latest-mac.yml`, `latest.yml`, ou `latest-linux.yml` existe**
- Os artefatos ainda mostram versão **0.1.0** no nome (deveria ser 0.1.2)
- O `electron-builder --publish always` NÃO gerou os metadados `.yml`

### Raiz do Problema
O `electron-builder --publish always` no Mac runner (macOS-latest) falha silenciosamente ao gerar/uploadar os metadados `.yml` quando:
1. A release já existe (duplicata) — ele pula o publish
2. O `GH_TOKEN` não tem permissão `contents: write` para a release
3. O conflito entre `--publish always` e o job `upload-release` que também adiciona artefatos

**Evidência:** O log do erro mostra `Please double check that your authentication token is correct` — mas o token está correto. O problema é que o `electron-builder` falha ao publicar os metadados e o job `upload-release` apenas adiciona binários (sem `.yml`).

---

## SOLUÇÕES POSSÍVEIS

### Solução A: Gerar manualmente os `.yml` no workflow (RECOMENDADA)

**Por que é a melhor:** Garante que os metadados existam independente de como o `electron-builder` publica.

**Como funciona:**
1. O build do Mac gera o `.dmg` (sem `--publish`)
2. Um step no workflow gera `latest-mac.yml` manualmente com base no `.dmg` gerado
3. O `upload-release` job adiciona O `.dmg` + `latest-mac.yml` à release

**Vantagens:**
- Não depende do `electron-builder publish` funcionar corretamente
- Funciona mesmo se a release já existir
- Controle total sobre o conteúdo do `.yml`

**Risco:**
- Precisa manter o `fileName` e `sha512` sincronizados com o artefato real

---

### Solução B: Usar `electron-updater` com URL customizada

**Como funciona:** No `main.js`, configurar o `feedUrl` do `autoUpdater` para apontar para um endpoint fixo (ex: um arquivo `latest-mac.yml` hospedado em S3 ou no próprio repo).

**Vantagens:**
- Não depende do GitHub Releases
- Mais controle

**Risco:**
- Infraestrutura extra necessária
- Não resolve o problema de upload do binário

---

### Solução C: Remover o job `upload-release` e deixar apenas `electron-builder --publish always`

**Como funciona:** Cada job de build publica diretamente na release sem intermediário.

**Vantagens:**
- Simplifica o workflow
- O `electron-builder` gera os `.yml` automaticamente

**Risco:**
- Conflito entre 3 jobs tentando publicar na mesma release simultaneamente
- Pode falhar se um job terminar antes do outro (race condition)

---

### Solução D: Usar `electronBuilder.forcePublish` ou `--publish onTag` (NÃO RECOMENDADA)

O `electron-builder` tem uma flag `--publish onTag` que publica apenas quando o commit tem tag. Isso não resolve o problema.

---

### Solução E: Remover `publish` do Mac/Linux e usar o `latest.yml` do Windows como fallback

**Como funciona:** Todos os clientes (Win/Mac/Linux) usam `latest.yml`. O `electron-updater` no Mac consegue ler `latest.yml` se configurado.

**Vantagens:**
- Simples
- Um único arquivo `.yml` para todas as plataformas

**Risco:**
- O `electron-updater` no Mac espera `latest-mac.yml` por padrão. Precisa customizar o provider.
- Não é a abordagem padrão

---

## DECISÃO: Solução A (Gerar manualmente)

### Implementação no workflow

```yaml
# No job build-mac, adicionar step ANTES do electron-builder:
- name: Generate latest-mac.yml
  run: |
    python3 -c "
    import hashlib, os, json, urllib.parse
    
    dmg_name = 'Ferramentas POSTECH-0.1.2.dmg'
    dmg_path = f'dist/{dmg_name}'
    with open(dmg_path, 'rb') as f:
        sha512 = hashlib.sha512(f.read()).hexdigest()
    
    metadata = {
        'version': '0.1.2',
        'releaseDate': datetime.utcnow().isoformat() + 'Z',
        'name': 'Ferramentas POSTECH v0.1.2',
        'fileName': dmg_name,
        'url': f'https://github.com/DyFuchs/Ferramentas_POSTECH_Electron/releases/download/v0.1.2/{urllib.parse.quote(dmg_name)}',
        'sha512': sha512
    }
    # Para electron-updater, o yml precisa de campos específicos
    yml_content = f'''version: 0.1.2
releaseDate: {metadata['releaseDate']}
name: Ferramentas POSTECH v0.1.2
notes: |
  - Fix Auto-update Mac
  - Fix Backend Rust race condition
  - Fix Linux AppImage
  - Icon 512x512 para Mac
fileName: {dmg_name}
url: {metadata['url']}
sha512: {sha512}
'''
    with open('dist/latest-mac.yml', 'w') as f:
        f.write(yml_content)
    print('Generated latest-mac.yml')
"
```

### No job `upload-release`
Adicionar `latest*.yml` aos arquivos a serem uploadados.

---

## PROBLEMA 2: Arquivos na release com versão errada (0.1.0 em vez de 0.1.2)

### Descrição
Os artefatos na release v0.1.2 mostram nome `Ferramentas POSTECH-0.1.0.dmg` e `.exe`.

### Causa
O `package.json` está com `version: "0.1.0"`. Precisa ser atualizado para `0.1.2` ANTES do build.

### Solução
1. Atualizar `package.json` para `"version": "0.1.2"`
2. Atualizar `package-lock.json` (`npm install`)
3. Commitar e criar nova tag `v0.1.3`

---

## PLANO DE AÇÃO COMPLETO

1. ✅ Criar este documento
2. ⚠️ Atualizar `package.json` para `0.1.2` (já deveria estar)
3. ⚠️ Gerar `latest*.yml` manualmente no workflow
4. ⚠️ Atualizar `upload-release` para incluir `*.yml`
5. ⚠️ Criar tag `v0.1.2` (force-update) ou `v0.1.3`
6. ⚠️ Aguardar build e verificar

---

## PROBLEMA 3: GitHub Actions — Heredoc `EOF` inválido no YAML

### Descrição
O workflow `.github/workflows/release.yml` usa `cat > file << EOF` (heredoc bash) dentro de um step YAML. O parser do GitHub Actions interpreta `EOF` como YAML, não como bash, causando erro:
```
Invalid workflow file
You have an error in your yaml syntax on line 46
```

### Causa
O `run: |` do YAML preserva indentação, mas o heredoc `<< EOF` precisa que o marcador `EOF` esteja na **coluna 0** (sem indentação). Com a indentação do step, o parser YAML não reconhece o heredoc corretamente.

### Solução
Usar `write_file` (ferramenta do Hermes) para gerar o arquivo `.yml` diretamente no sistema de arquivos, em vez de usar heredoc bash no workflow. Ou usar `printf` em vez de heredoc:
```yaml
- name: Generate latest.yml
  shell: bash
  run: |
    printf 'version: 0.1.2\nreleaseDate: %s\nname: Ferramentas POSTECH v0.1.2\nnotes: |\n  - Fix: Auto-update\nfileName: %s\nurl: https://github.com/.../v0.1.2/%s\nsha512: %s\n' \
      "$(date -u +%Y-%m-%dT%H:%M:%S.000Z)" \
      "$EXE_NAME" "$EXE_NAME" "$SHA" > dist/latest.yml
```

### Referência
- [GitHub Actions: Using heredoc in YAML](https://docs.github.com/en/actions/using-jobs/workflow-syntax-for-github-actions#example-using-a-multiline-string) — Heredoc funciona, mas o marcador precisa estar alinhado com a indentação do bloco `run:`

---

## PROBLEMA 4: `update-not-available` sem feedback visual na UI

### Descrição
Quando o auto-update verifica e não encontra novas versões, o log mostra `[UPDATER] Atualizado`, mas o usuário não vê nenhuma notificação na UI. Isso confunde — parece que o update falhou, quando na verdade só não há atualização disponível.

### Causa
O `main.js` tem um listener para `update-not-available` que apenas faz `console.log('[UPDATER] Atualizado')`. Não envia nenhuma mensagem para a janela principal.

### Solução
Enviar evento IPC `update-not-available` para o renderer e mostrar um toast na UI:
```javascript
// main.js
autoUpdater.on('update-not-available', function(info) {
  console.log('[UPDATER] Atualizado — versão mais recente:', info.version);
  if (mainWindow && !mainWindow.isDestroyed()) {
    mainWindow.webContents.send('update-not-available', info);
  }
});
```

```javascript
// index.html (renderer)
window.electronAPI.onUpdateNotAvailable(function(info) {
  toast('Seu app está na versão mais recente — não foram encontradas atualizações.', 'info');
});
```

---

## PROBLEMA 5: EGL `eglQueryDeviceAttribEXT: Bad attribute` repetitivo

### Descrição
O log mostra repetidamente:
```
[PID:0625/HHMMSS.xxxxxx:ERROR:gl_display.cc(497)] EGL Driver message (Error) eglQueryDeviceAttribEXT: Bad attribute.
```

### Diagnóstico
- **Não é um bug do app** — é um warning conhecido do Chromium/Electron em Macs com GPU AMD Radeon
- Afeta Macs com Radeon Pro 560X, 555X, Vega, etc.
- O erro vem do driver OpenGL/EGL do Chromium ao consultar atributos de GPU não suportados
- **Não afeta funcionalidade** — o app funciona normalmente
- O erro se repete porque o Chromium consulta o EGL a cada frame de renderização

### Solução (se quiser suprimir)
Adicionar flag no `main.js` para desabilitar GPU acceleration ou suprimir logs EGL:
```javascript
// main.js — antes de app.whenReady()
app.commandLine.appendSwitch('disable-gpu-compositing');
app.commandLine.appendSwitch('disable-gpu');
```

**NÃO RECOMENDADO** — desabilitar GPU degrada performance. O correto é ignorar o warning.

### Referência
- [Electron issue #43415](https://github.com/electron/electron/issues/43415) — Bug report confirmado, fechado como "won't fix" (comportamento esperado para GPUs AMD)
- [Stack Overflow — eglQueryDeviceAttribEXT: Bad attribute](https://stackoverflow.com/questions/57090258/how-to-address-the-error-egl-driver-message-error-eglquerydeviceattribext-bad)

---

## REFERÊNCIAS

- [electron-builder troubleshooting](https://www.electron.build/docs/troubleshooting/) — "latest.yml / latest-mac.yml / latest-linux.yml only when publishing"
- [electron-builder issue #9155](https://github.com/electron-userland/electron-builder/issues/9155) — Mac runner throwing errors on random dmg builds
- [electron-builder issue #4942](https://github.com/electron-userland/electron-builder/issues/4942) — Auto updater error "latest-mac.yml not found"
- [nickbeaulieu.dev — How we deploy our Electron app](https://nickbeaulieu.dev/posts/deploy-electron-app) — Estratégia de merge dos `.yml` files
- [Electron issue #43415](https://github.com/electron/electron/issues/43415) — eglQueryDeviceAttribEXT: Bad attribute (AMD GPUs)
