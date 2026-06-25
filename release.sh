#!/bin/bash
# release.sh — Protocolo de Release para Organizador POSTECH
# Uso: ./release.sh [nova-versao] [mensagem]
# Exemplo: ./release.sh 0.1.4 "Fix no updater e melhorias no relatório"

set -e

# ============ CONFIGURAÇÃO ============
REPO="DyFuchs/Ferramentas_POSTECH_Electron"
MAIN_BRANCH="main"
VERSION_FILE="package.json"

# ============ PARÂMETROS ============
NEW_VERSION=${1:-""}
COMMIT_MSG=${2:-"Release v${NEW_VERSION}"}

if [ -z "$NEW_VERSION" ]; then
  echo "❌ Uso: ./release.sh [nova-versao] [mensagem]"
  echo "   Exemplo: ./release.sh 0.1.4 'Fix no updater'"
  exit 1
fi

# Valida formato de versão
if [[ ! "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "❌ Formato inválido. Use: X.Y.Z (ex: 0.1.4)"
  exit 1
fi

echo "============================================"
echo "  Organizador POSTECH — Protocolo de Release"
echo "============================================"
echo ""
echo "Nova versão: $NEW_VERSION"
echo "Mensagem: $COMMIT_MSG"
echo ""

# ============ PASSO 1: Verificar estado do repositório ============
echo "[1/6] Verificando estado do repositório..."

if ! git diff --quiet; then
  echo "⚠️  Há mudanças não commitadas. Commitando antes de prosseguir..."
  git add -A
  git commit -m "chore: antes do release v${NEW_VERSION}"
fi

CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "$MAIN_BRANCH" ]; then
  echo "⚠️  Branch atual é '$CURRENT_BRANCH'. Mudando para '$MAIN_BRANCH'..."
  git checkout "$MAIN_BRANCH"
fi

# ============ PASSO 2: Atualizar arquivos de versão ============
echo "[2/6] Atualizando arquivos de versão..."

# package.json
sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"${NEW_VERSION}\"/" "$VERSION_FILE"
rm -f "$VERSION_FILE.bak"
echo "  ✅ package.json: $NEW_VERSION"

# src/index.html — APP_VERSION
sed -i.bak "s/const APP_VERSION = '[^']*'/const APP_VERSION = '${NEW_VERSION}'/" src/index.html
rm -f src/index.html.bak
echo "  ✅ src/index.html APP_VERSION: $NEW_VERSION"

# src/index.html — sidebar-version (hardcoded fallback)
sed -i.bak "s/id=\"sidebar-version\">[^<]*/id=\"sidebar-version\">${NEW_VERSION}/" src/index.html
rm -f src/index.html.bak
echo "  ✅ src/index.html sidebar-version: $NEW_VERSION"

# src/index.html — credits-version (hardcoded fallback)
sed -i.bak "s/id=\"credits-version\">[^<]*/id=\"credits-version\">${NEW_VERSION}/" src/index.html
rm -f src/index.html.bak
echo "  ✅ src/index.html credits-version: $NEW_VERSION"

# ============ PASSO 3: Verificar que não há versões antigas ============
echo "[3/6] Verificando consistência de versão..."

OLD_VERSIONS=$(grep -rn "0\.1\.2\|0\.1\.0\|1\.0\.0" src/ package.json main.js 2>/dev/null | grep -v node_modules | grep -v "\.git/" || true)
if [ -n "$OLD_VERSIONS" ]; then
  echo "⚠️  Encontradas possíveis referências a versões antigas:"
  echo "$OLD_VERSIONS"
  echo ""
  read -p "Continuar mesmo assim? (s/N) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    echo "❌ Release cancelado."
    exit 1
  fi
else
  echo "  ✅ Nenhuma referência a versão antiga encontrada."
fi

# ============ PASSO 4: Commit e Tag ============
echo "[4/6] Criando commit e tag..."

git add -A
git commit -m "v${NEW_VERSION} - ${COMMIT_MSG}"

git tag -d "v${NEW_VERSION}" 2>/dev/null || true  # Remove tag local se existir
git push origin ":v${NEW_VERSION}" 2>/dev/null || true  # Remove tag remota se existir

git tag "v${NEW_VERSION}"
echo "  ✅ Tag v${NEW_VERSION} criada."

# ============ PASSO 5: Push ============
echo "[5/6] Enviando para o GitHub..."

git push origin "$MAIN_BRANCH"
git push origin "v${NEW_VERSION}"
echo "  ✅ Push concluído."

# ============ PASSO 6: Resumo ============
echo "[6/6] release preparado com sucesso!"
echo ""
echo "============================================"
echo "  Release v${NEW_VERSION} enviada!"
echo "============================================"
echo ""
echo "Próximos passos:"
echo "  1. Acompanhar a build: https://github.com/${REPO}/actions"
echo "  2. Verificar se os assets foram gerados com 'v${NEW_VERSION}' no nome"
echo "  3. Verificar se o latest.yml tem 'version: ${NEW_VERSION}'"
echo "  4. Testar o updater na versão instalada"
echo ""
echo "Se a build falhar, corrija e execute novamente:"
echo "  ./release.sh ${NEW_VERSION} \"${COMMIT_MSG}\""
echo ""
