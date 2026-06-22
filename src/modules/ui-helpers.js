// src/modules/ui-helpers.js
// Utilitarios de UI: toast, modals, confirm

var pendingAction = null;

function toast(msg, type) {
    type = type || 'info';
    var container = document.getElementById('toast-container');
    if (!container) return;
    var el = document.createElement('div');
    el.className = 'toast ' + type;
    el.textContent = msg;
    container.appendChild(el);
    setTimeout(function() { if (el.parentNode) el.parentNode.removeChild(el); }, 3500);
}

function showConfirmModal(title, message, details, action) {
    document.getElementById('modal-title').textContent = title;
    document.getElementById('modal-message').textContent = message;
    document.getElementById('modal-details').innerHTML = details || '';
    document.getElementById('confirm-modal').classList.add('active');
    pendingAction = action;
}

function closeModal() {
    document.getElementById('confirm-modal').classList.remove('active');
    pendingAction = null;
}

function confirmModal() {
    if (pendingAction) pendingAction();
    closeModal();
}

// Expor ao window
window.toast = toast;
window.showConfirmModal = showConfirmModal;
window.closeModal = closeModal;
window.confirmModal = confirmModal;
