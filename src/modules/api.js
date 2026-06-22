// src/modules/api.js
// Comunicacao com Rust via IPC do Electron
// Unica fonte de verdade para invokeRust e initAPI

var electronAvailable = false;
var rustListeners = [];
var rustRequestQueue = [];
var rustRequestInProgress = false;

function initAPI() {
    try {
        if (window.electronAPI) {
            electronAvailable = true;
            console.log('[API] Electron API disponivel');
            window.electronAPI.onRustMessage(function(msg) {
                console.log('[RENDERER] Mensagem do Rust recebida:', msg);
                for (var i = 0; i < rustListeners.length; i++) {
                    rustListeners[i](msg);
                }
            });
            console.log('[RENDERER] Listener registrado');
        } else {
            throw new Error('window.electronAPI nao encontrado');
        }
    } catch(e) {
        console.log('Electron API nao disponivel:', e.message);
        electronAvailable = false;
    }
}

function isElectronAvailable() {
    return electronAvailable;
}

async function invokeRust(cmd, args) {
    args = args || {};
    return new Promise(function(resolve, reject) {
        if (!electronAvailable) {
            reject(new Error('Electron API nao disponivel'));
            return;
        }
        var id = Date.now() + Math.random();
        var timeout = setTimeout(function() {
            rustListeners = rustListeners.filter(function(l) { return l._id !== id; });
            reject(new Error('Timeout aguardando resposta do backend'));
        }, 120000);
        var handler = function(msg) {
            if (msg.type === 'success' || msg.type === 'error') {
                clearTimeout(timeout);
                rustListeners = rustListeners.filter(function(l) { return l._id !== id; });
                if (msg.type === 'success') resolve(msg.data);
                else reject(new Error(msg.message));
            }
        };
        handler._id = id;
        rustListeners.push(handler);
        window.electronAPI.sendCommand({ cmd: cmd, id: id, args: args });
    });
}

// Aliases para compatibilidade com codigo legado
window.initAPI = initAPI;
window.invokeRust = invokeRust;
window.isElectronAvailable = isElectronAvailable;
