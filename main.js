const { app, BrowserWindow, ipcMain, dialog, clipboard } = require('electron');
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

let mainWindow;
let rustBackend = null;

// ============ RUST SIDECAR ============

function startRustBackend() {
  const isDev = !app.isPackaged;
  let backendPath;

  if (isDev) {
    backendPath = path.join(__dirname, 'rust-backend', 'target', 'release', 'organizador-postech-backend');
  } else {
    backendPath = path.join(process.resourcesPath, 'backend', 'organizador-postech-backend');
  }

  // Adiciona .exe no Windows
  if (process.platform === 'win32') {
    backendPath += '.exe';
  }

  console.log('[DEBUG] Caminho do backend:', backendPath);

  // Verifica se o binário existe
  if (!fs.existsSync(backendPath)) {
    console.error('[ERRO] Backend Rust não encontrado:', backendPath);
    // Tenta compilar automaticamente em dev
    if (isDev) {
      console.log('[INFO] Tentando compilar o backend Rust...');
      const { execSync } = require('child_process');
      try {
        execSync('cd rust-backend && cargo build --release', { stdio: 'inherit' });
      } catch (e) {
        console.error('[ERRO] Falha ao compilar backend:', e.message);
        return;
      }
    } else {
      return;
    }
  }

  console.log('[INFO] Iniciando backend Rust:', backendPath);
  rustBackend = spawn(backendPath, [], {
    stdio: ['pipe', 'pipe', 'pipe']
  });

  rustBackend.stderr.on('data', (data) => {
    const raw = data.toString();
    console.log('[DEBUG stderr raw]', JSON.stringify(raw));
    const lines = raw.trim().split('\n');
    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed) continue;
      // Tenta parsear como JSON (resposta do Rust)
      try {
        const msg = JSON.parse(trimmed);
        if (msg.type === 'success' || msg.type === 'error') {
          console.log('[DEBUG] Resposta JSON do Rust:', msg);
          // É uma resposta do backend, encaminha para o renderer
          if (mainWindow && !mainWindow.isDestroyed()) {
            mainWindow.webContents.send('rust-message', msg);
          }
          continue;
        }
      } catch (e) {
        // Não é JSON, é log normal do Rust
      }
      console.error('[RUST stderr]', trimmed);
    }
  });

  rustBackend.stdout.on('data', (data) => {
    console.log('[RUST stdout]', data.toString().trim());
  });

  rustBackend.on('close', (code) => {
    console.log('[RUST] Processo encerrado com código:', code);
    rustBackend = null;
  });

  rustBackend.on('error', (err) => {
    console.error('[RUST] Erro ao iniciar:', err.message);
  });
}

function sendToRust(payload) {
  if (rustBackend && rustBackend.stdin.writable) {
    const json = JSON.stringify(payload);
    console.log('[DEBUG] Enviando para Rust:', json);
    rustBackend.stdin.write(json + '\n');
  } else {
    console.error('[ERRO] Backend Rust não está rodando');
    if (mainWindow && !mainWindow.isDestroyed()) {
      mainWindow.webContents.send('rust-message', {
        type: 'error',
        id: payload.id,
        message: 'Backend Rust não está rodando'
      });
    }
  }
}

// ============ JANELA PRINCIPAL ============

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    minWidth: 900,
    minHeight: 600,
    title: 'Ferramentas POSTECH',
    icon: path.join(__dirname, 'assets', 'icon.ico'),
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false
    }
  });

  mainWindow.loadFile(path.join(__dirname, 'src', 'index.html'));

  // Abre DevTools em desenvolvimento
  if (!app.isPackaged) {
    mainWindow.webContents.openDevTools();
  }
}

// ============ IPC HANDLERS ============

ipcMain.handle('select-folder', async () => {
  const result = await dialog.showOpenDialog(mainWindow, {
    properties: ['openDirectory']
  });
  return result.canceled ? null : result.filePaths[0];
});

ipcMain.handle('clipboard-write', async (event, text) => {
  clipboard.writeText(text);
  return true;
});

ipcMain.handle('clipboard-write-html', async (event, html, text) => {
  clipboard.write({
    text: text || html,
    html: html
  });
  return true;
});

// Abrir arquivo/pasta no explorador
ipcMain.handle('open-path', async (event, filePath) => {
  const { shell } = require('electron');
  await shell.openPath(filePath);
  return true;
});

ipcMain.on('rust-command', (event, payload) => {
  sendToRust(payload);
});

// ============ AUTO-UPDATE ============
// Usa electron-updater para verificar e baixar atualizações do GitHub Releases

function setupAutoUpdate() {
  try {
    var updater = require('electron-updater');
    var autoUpdater = updater.autoUpdater;

    autoUpdater.logger = console;

    // Verifica atualizações ao iniciar (após 5s para não interferir no startup)
    setTimeout(function() {
      console.log('[UPDATER] Verificando atualizações...');
      autoUpdater.checkForUpdatesAndNotify().catch(function(e) {
        console.log('[UPDATER] Erro ao verificar:', e.message);
      });
    }, 5000);

    autoUpdater.on('checking-for-update', function() {
      console.log('[UPDATER] Verificando...');
    });

    autoUpdater.on('update-available', function(info) {
      console.log('[UPDATER] Disponível:', info.version);
      if (mainWindow && !mainWindow.isDestroyed()) {
        mainWindow.webContents.send('update-available', info);
      }
    });

    autoUpdater.on('update-not-available', function() {
      console.log('[UPDATER] Atualizado');
    });

    autoUpdater.on('update-downloaded', function(info) {
      console.log('[UPDATER] Baixada:', info.version);
      if (mainWindow && !mainWindow.isDestroyed()) {
        mainWindow.webContents.send('update-downloaded', info);
      }
    });

    autoUpdater.on('error', function(err) {
      console.error('[UPDATER] Erro:', err.message);
    });

    // Verifica a cada 30 minutos
    setInterval(function() {
      autoUpdater.checkForUpdates().catch(function() {});
    }, 30 * 60 * 1000);

    ipcMain.handle('check-for-updates', function() {
      return autoUpdater.checkForUpdatesAndNotify();
    });

    ipcMain.handle('download-update', function() {
      return autoUpdater.downloadUpdate();
    });

    ipcMain.handle('quit-and-install', function() {
      autoUpdater.quitAndInstall();
    });

    // Abre URL no navegador externo
    ipcMain.handle('open-url', async (event, url) => {
      const { shell } = require('electron');
      await shell.openExternal(url);
      return true;
    });

  } catch(e) {
    console.log('[UPDATER] electron-updater indisponível');
  }
}

// ============ APP LIFECYCLE ============

app.whenReady().then(() => {
  startRustBackend();
  createWindow();
  setupAutoUpdate();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (rustBackend) {
    rustBackend.kill();
    rustBackend = null;
  }
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('before-quit', () => {
  if (rustBackend) {
    rustBackend.kill();
  }
});
