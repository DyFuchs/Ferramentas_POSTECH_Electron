const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  selectFolder: () => ipcRenderer.invoke('select-folder'),
  sendCommand: (payload) => ipcRenderer.send('rust-command', payload),
  clipboardWrite: (text) => ipcRenderer.invoke('clipboard-write', text),
  clipboardWriteHtml: (html, text) => ipcRenderer.invoke('clipboard-write-html', html, text),
  openPath: (filePath) => ipcRenderer.invoke('open-path', filePath),
  openUrl: (url) => ipcRenderer.invoke('open-url', url),
  onRustMessage: (callback) => {
    ipcRenderer.on('rust-message', (_event, data) => {
      callback(data);
    });
  },
  // Auto-update
  checkForUpdates: () => ipcRenderer.invoke('check-for-updates'),
  downloadUpdate: () => ipcRenderer.invoke('download-update'),
  quitAndInstall: () => ipcRenderer.invoke('quit-and-install'),
  onUpdateAvailable: (callback) => {
    ipcRenderer.on('update-available', (_event, data) => { callback(data); });
  },
  onUpdateDownloaded: (callback) => {
    ipcRenderer.on('update-downloaded', (_event, data) => { callback(data); });
  },
  onUpdateError: (callback) => {
    ipcRenderer.on('update-error', (_event, data) => { callback(data); });
  },
  platform: process.platform
});
