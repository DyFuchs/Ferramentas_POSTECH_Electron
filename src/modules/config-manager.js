// src/modules/config-manager.js
// Gerenciamento de configuracoes - salvar/carregar do Rust e localStorage

var activeProfile = 'POSTECH Escuro';
var colorProfiles = {};
var shortcuts = {};
var electronAvailable = false;

function buildConfig() {
    var config = {
        last_used_path: document.getElementById('global-path') ? document.getElementById('global-path').value || '' : '',
        confirm_required: document.getElementById('cfg-confirm') ? document.getElementById('cfg-confirm').checked : true,
        theme: document.documentElement.getAttribute('data-theme') || 'dark',
        font_size: document.getElementById('cfg-fontsize') ? document.getElementById('cfg-fontsize').value || '14' : '14',
        font_weight: document.getElementById('cfg-fontweight') ? document.getElementById('cfg-fontweight').value || '600' : '600',
        font_style: document.getElementById('cfg-fontstyle') ? document.getElementById('cfg-fontstyle').value || 'normal' : 'normal',
        report_html: document.getElementById('cfg-report-html') ? document.getElementById('cfg-report-html').checked : true,
        report_csv: document.getElementById('cfg-report-csv') ? document.getElementById('cfg-report-csv').checked : true,
        report_txt: document.getElementById('cfg-report-txt') ? document.getElementById('cfg-report-txt').checked : true
    };
    config.shortcuts = shortcuts;
    config.activeProfile = activeProfile;
    config.colorProfiles = colorProfiles;
    var autoSaveEl = document.getElementById('cfg-autosave');
    if (autoSaveEl) config.auto_save_minutos = parseInt(autoSaveEl.value);
    return config;
}

function saveConfigSilent(cfg) {
    var clean = {};
    for (var k in cfg) {
        if (cfg.hasOwnProperty(k) && cfg[k] !== null && cfg[k] !== undefined) {
            clean[k] = cfg[k];
        }
    }
    clean.shortcuts = shortcuts;
    clean.activeProfile = activeProfile;
    clean.colorProfiles = colorProfiles;
    if (electronAvailable && window.invokeRust) {
        window.invokeRust('salvarConfig', { config: clean }).catch(function(e) {
            console.log('Erro ao salvar no backend, salvando local:', e);
            localStorage.setItem('app_config', JSON.stringify(clean));
        });
    } else {
        localStorage.setItem('app_config', JSON.stringify(clean));
    }
}

async function saveConfig() {
    var config = buildConfig();

    var btn = document.querySelector('button[onclick="saveConfig()"]');
    var originalText = btn ? btn.textContent : '';
    if (btn) btn.textContent = '\u23F3 Salvando...';

    if (electronAvailable && window.invokeRust) {
        try {
            await window.invokeRust('salvarConfig', { config: config });
            if (btn) btn.textContent = '\u2705 Salvo!';
            if (window.toast) window.toast('\u2713 Configurações salvas com sucesso!', 'success');
        } catch(e) {
            console.log('Erro ao salvar no backend, salvando local:', e);
            localStorage.setItem('app_config', JSON.stringify(config));
            if (btn) btn.textContent = '\u2705 Salvo (local)!';
            if (window.toast) window.toast('\u2713 Configurações salvas (local)!', 'success');
        }
    } else {
        localStorage.setItem('app_config', JSON.stringify(config));
        if (btn) btn.textContent = '\u2705 Salvo (local)!';
        if (window.toast) window.toast('\u2713 Configurações salvas (local)!', 'success');
    }

    setTimeout(function() { if (btn) btn.textContent = originalText; }, 2000);
}


async function loadConfig() {
    var config = null;
    if (electronAvailable && window.invokeRust) {
        try {
            config = await window.invokeRust('lerConfig');
        } catch(e) {
            console.log('Erro ao ler config do backend');
        }
    }
    if (!config) {
        var saved = localStorage.getItem('organizador_config');
        if (saved) {
            try { config = JSON.parse(saved); } catch(e) { config = null; }
        }
    }
    if (config && typeof config === 'object') {
        if (config.last_used_path && document.getElementById('global-path')) {
            document.getElementById('global-path').value = config.last_used_path;
        }
        var confirmVal = config.confirm_required !== undefined ? config.confirm_required : config.confirmRequired;
        if (confirmVal !== undefined && confirmVal !== null) {
            var el = document.getElementById('cfg-confirm');
            if (el) el.checked = confirmVal;
        }
        if (config.theme === 'light') {
            document.documentElement.setAttribute('data-theme', 'light');
        }
        if (config.font_size && document.getElementById('cfg-fontsize')) {
            document.getElementById('cfg-fontsize').value = config.font_size;
        }
        if (config.font_weight && document.getElementById('cfg-fontweight')) {
            document.getElementById('cfg-fontweight').value = config.font_weight;
        }
        if (config.font_style && document.getElementById('cfg-fontstyle')) {
            document.getElementById('cfg-fontstyle').value = config.font_style;
        }
        if (config.activeProfile) activeProfile = config.activeProfile;
        if (config.colorProfiles) {
            colorProfiles = JSON.parse(JSON.stringify(config.colorProfiles));
        }
        if (config.shortcuts) shortcuts = config.shortcuts;

        // Report formats with fallback
        const reportConfigs = [
            { id: 'cfg-report-html', key: 'report_html' },
            { id: 'cfg-report-csv', key: 'report_csv' },
            { id: 'cfg-report-txt', key: 'report_txt' }
        ];
        reportConfigs.forEach(rc => {
            const el = document.getElementById(rc.id);
            if (el) {
                if (config[rc.key] !== undefined) {
                    el.checked = config[rc.key];
                } else {
                    el.checked = true; // Fallback for old versions
                }
            }
        });
    }
    return config;
}
function applyFontSettings() {
    var size = document.getElementById('cfg-fontsize') ? document.getElementById('cfg-fontsize').value || '14' : '14';
    var weight = document.getElementById('cfg-fontweight') ? document.getElementById('cfg-fontweight').value || '600' : '600';
    var style = document.getElementById('cfg-fontstyle') ? document.getElementById('cfg-fontstyle').value || 'normal' : 'normal';
    document.documentElement.style.setProperty('--font-size', size + 'px');
    document.documentElement.style.setProperty('--font-weight', weight);
    document.documentElement.style.setProperty('--font-style', style);
}

function applyFontPreset(preset) {
    var presets = {
        compact: { size: '12', weight: '600', style: 'normal' },
        normal: { size: '14', weight: '600', style: 'normal' },
        large: { size: '16', weight: '700', style: 'normal' },
        bold: { size: '14', weight: '800', style: 'normal' }
    };
    var p = presets[preset] || presets.normal;
    if (document.getElementById('cfg-fontsize')) document.getElementById('cfg-fontsize').value = p.size;
    if (document.getElementById('cfg-fontweight')) document.getElementById('cfg-fontweight').value = p.weight;
    if (document.getElementById('cfg-fontstyle')) document.getElementById('cfg-fontstyle').value = p.style;
    applyFontSettings();
}

// Expor ao window
window.buildConfig = buildConfig;
window.saveConfig = saveConfig;
window.saveConfigSilent = saveConfigSilent;
window.loadConfig = loadConfig;
window.applyFontSettings = applyFontSettings;
window.applyFontPreset = applyFontPreset;
