// src/modules/utils.js
// Utilitarios gerais - funcoes puras sem dependencias

function escapeHtml(s) {
    if (!s) return '';
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

function cleanVideoName(nome) {
    return nome
        .replace(/^.*\//, '')
        .replace(/\.[^.]+$/, '')
        .replace(/[\s_-]+[Vv]_?\d+\s*$/, '')
        .trim();
}

function isTextInputElement(el) {
    if (!el) return false;
    var tag = el.tagName.toLowerCase();
    if (tag === 'input' || tag === 'textarea' || tag === 'select') return true;
    if (el.isContentEditable) return true;
    return false;
}

function fmtDuration(secs) {
    var h = Math.floor(secs / 3600);
    var m = Math.floor((secs % 3600) / 60);
    var s = Math.floor(secs % 60);
    return ('0' + h).slice(-2) + ':' + ('0' + m).slice(-2) + ':' + ('0' + s).slice(-2);
}

// Expor ao window
window.escapeHtml = escapeHtml;
window.cleanVideoName = cleanVideoName;
window.isTextInputElement = isTextInputElement;
window.fmtDuration = fmtDuration;
