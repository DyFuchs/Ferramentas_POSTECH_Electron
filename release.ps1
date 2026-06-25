# release.ps1 — Protocolo de Release para Organizador POSTECH (PowerShell)
# Uso: .\release.ps1 -Version "0.1.4" -Message "Fix no updater e melhorias"
# Exemplo: .\release.ps1 -Version "0.1.4" -Message "Fix no updater"

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,

    [Parameter(Mandatory=$true)]
    [string]$Message
)

$ErrorActionPreference = "Stop"

# ============ CONFIGURAÇÃO ============
$MAIN_BRANCH = "main"
$VERSION_FILE = "package.json"

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Organizador POSTECH — Protocolo de Release" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Nova versão: $Version" -ForegroundColor Green
Write-Host "Mensagem: $Message" -ForegroundColor Green
Write-Host ""

# ============ PASSO 1: Verificar estado do repositório ============
Write-Host "[1/6] Verificando estado do repositório..." -ForegroundColor Yellow

$status = git status --porcelain
if ($status) {
    Write-Host "  Mudancas encontradas. Commitando antes de prosseguir..." -ForegroundColor Yellow
    git add -A
    git commit -m "chore: antes do release v${Version}"
}

$currentBranch = git branch --show-current
if ($currentBranch -ne $MAIN_BRANCH) {
    Write-Host "  Mudando de '$currentBranch' para '$MAIN_BRANCH'..." -ForegroundColor Yellow
    git checkout "$MAIN_BRANCH"
}

# ============ PASSO 2: Atualizar arquivos de versão ============
Write-Host "[2/6] Atualizando arquivos de versão..." -ForegroundColor Yellow

# package.json
$pkgContent = Get-Content $VERSION_FILE -Raw; $pkgContent = $pkgContent -replace '"version":\s*"[^"]*"', "\"version\": \"$Version\""; Set-Content $VERSION_FILE $pkgContent -NoNewline
Write-Host "  package.json: $Version" -ForegroundColor Green

# src/index.html — APP_VERSION
$htmlContent = Get-Content "src/index.html" -Raw; $htmlContent = $htmlContent -replace "const APP_VERSION = '[^']*'", "const APP_VERSION = '$Version'"; Set-Content "src/index.html" $htmlContent -NoNewline
Write-Host "  src/index.html APP_VERSION: $Version" -ForegroundColor Green

# src/index.html — sidebar-version
$htmlContent = Get-Content "src/index.html" -Raw; $htmlContent = $htmlContent -replace 'id="sidebar-version">[^<]*', "id=\"sidebar-version\">$Version"; Set-Content "src/index.html" $htmlContent -NoNewline
Write-Host "  src/index.html sidebar-version: $Version" -ForegroundColor Green

# src/index.html — credits-version
$htmlContent = Get-Content "src/index.html" -Raw; $htmlContent = $htmlContent -replace 'id="credits-version">[^<]*', "id=\"credits-version\">$Version"; Set-Content "src/index.html" $htmlContent -NoNewline
Write-Host "  src/index.html credits-version: $Version" -ForegroundColor Green

# ============ PASSO 3: Verificar versões antigas ============
Write-Host "[3/6] Verificando consistência de versão..." -ForegroundColor Yellow

$oldVersions = Select-String -Path "src/*","package.json","main.js" -Pattern "0\.1\.2|0\.1\.0|1\.0\.0" -SimpleMatch
if ($oldVersions) {
    Write-Host "  Encontradas possiveis referencias a versoes antigas:" -ForegroundColor Red
    $oldVersions | ForEach-Object { Write-Host "    $($_.Path): $($_.Line.Substring(0, [Math]::Min(80, $_.Line.Length)))" }
    Write-Host ""
    $continue = Read-Host "  Continuar mesmo assim? (s/N)"
    if ($continue -ne 's') {
        Write-Host "Release cancelada." -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  Nenhuma referencia a versao antiga encontrada." -ForegroundColor Green
}

# ============ PASSO 4: Commit e Tag ============
Write-Host "[4/6] Criando commit e tag..." -ForegroundColor Yellow

git add -A
git commit -m "v${Version} ${Message}"

# Remover tag se existir (para rebuild)
$tagExists = git tag -l "v${Version}" 2>$null
if ($tagExists) {
    Write-Host "  Removendo tag v${Version} existente..." -ForegroundColor Yellow
    git tag -d "v${Version}" 2>$null
    git push origin ":v${Version}" 2>$null
}

git tag "v${Version}"
Write-Host "  Tag v${Version} criada." -ForegroundColor Green

# ============ PASSO 5: Push ============
Write-Host "[5/6] Enviando para o GitHub..." -ForegroundColor Yellow

git push origin "$MAIN_BRANCH"
git push origin "v${Version}"
Write-Host "  Push concluido." -ForegroundColor Green

# ============ PASSO 6: Resumo ============
Write-Host "[6/6] Release preparada com sucesso!" -ForegroundColor Yellow
Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Release v${Version} enviada!" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Proximos passos:" -ForegroundColor White
Write-Host "  1. Acompanhar a build: https://github.com/DyFuchs/Ferramentas_POSTECH_Electron/actions"
Write-Host "  2. Verificar se os assets foram gerados com 'v${Version}' no nome" -ForegroundColor White
Write-Host "  3. Verificar se o latest.yml tem 'version: ${Version}'" -ForegroundColor White
Write-Host "  4. Testar o updater na versao instalada" -ForegroundColor White
Write-Host ""
