@echo off
echo ============================================
echo  Organizador POSTECH - Build Script
echo ============================================
echo.

echo [1/2] Compilando backend Rust...
cd rust-backend
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo [ERRO] Falha ao compilar Rust!
    pause
    exit /b 1
)
cd..

echo.
echo [2/2] Gerando instalador Windows...
npm run build:win
if %ERRORLEVEL% neq 0 (
    echo [ERRO] Falha ao gerar instalador!
    pause
    exit /b 1
)

echo.
echo ============================================
echo  BUILD CONCLUIDO COM SUCESSO!
echo ============================================
echo.
echo Instalador: dist\Organizador POSTECH_1.0.0_x64-setup.exe
echo.
pause
