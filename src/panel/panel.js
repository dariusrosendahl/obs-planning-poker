const CARD_VALUES = ['0', '1', '2', '3', '5', '8', '13', '21', '?', '\u2615', '\u221E'];

const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;

const grid = document.getElementById('card-grid');
const currentValue = document.getElementById('current-value');
const btnPrev = document.getElementById('btn-prev');
const btnReveal = document.getElementById('btn-reveal');
const btnNext = document.getElementById('btn-next');
const btnPin = document.getElementById('btn-pin');
const btnSettings = document.getElementById('btn-settings');
const settingsPanel = document.getElementById('settings-panel');
const portInput = document.getElementById('port-input');
const btnSavePort = document.getElementById('btn-save-port');
const obsUrl = document.getElementById('obs-url');
const restartNotice = document.getElementById('restart-notice');

let currentPort = 7777;

// Build card buttons
CARD_VALUES.forEach((value) => {
  const btn = document.createElement('button');
  btn.className = 'card-btn';
  btn.textContent = value;
  btn.dataset.value = value;
  btn.addEventListener('click', () => sendCommand('set_card', { value }));
  grid.appendChild(btn);
});

// Controls
btnPrev.addEventListener('click', () => sendCommand('prev_card'));
btnNext.addEventListener('click', () => sendCommand('next_card'));
btnReveal.addEventListener('click', () => sendCommand('toggle_reveal'));

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
  // Ignore when typing in input
  if (e.target.tagName === 'INPUT') return;

  if (e.key === 'ArrowLeft') {
    sendCommand('prev_card');
  } else if (e.key === 'ArrowRight') {
    sendCommand('next_card');
  } else if (e.key === ' ') {
    e.preventDefault();
    sendCommand('toggle_reveal');
  } else if (e.key >= '0' && e.key <= '8') {
    const index = parseInt(e.key, 10);
    if (index < CARD_VALUES.length) {
      sendCommand('set_card', { value: CARD_VALUES[index] });
    }
  }
});

// Tauri IPC
async function sendCommand(cmd, args = {}) {
  try {
    const state = await invoke(cmd, args);
    updatePanel(state);
  } catch (err) {
    console.error('IPC error:', err);
  }
}

function updatePanel(state) {
  currentValue.textContent = state.value;
  document.querySelectorAll('.card-btn').forEach((btn) => {
    btn.classList.toggle('active', btn.dataset.value === state.value);
  });
  btnReveal.classList.toggle('active', state.revealed);
}

// Load initial state and port
sendCommand('get_state');
invoke('get_port').then((port) => {
  currentPort = port;
  portInput.value = port;
  obsUrl.textContent = `http://localhost:${port}/card`;
});

// Settings toggle
btnSettings.addEventListener('click', () => {
  settingsPanel.classList.toggle('hidden');
  btnSettings.classList.toggle('active');
});

// Save port
btnSavePort.addEventListener('click', async () => {
  const newPort = parseInt(portInput.value, 10);
  if (newPort < 1024 || newPort > 65535 || isNaN(newPort)) return;

  try {
    await invoke('set_port', { port: newPort });
    obsUrl.textContent = `http://localhost:${newPort}/card`;
    if (newPort !== currentPort) {
      restartNotice.classList.remove('hidden');
    } else {
      restartNotice.classList.add('hidden');
    }
  } catch (err) {
    console.error('Failed to save port:', err);
  }
});

// Always on top (native window pinning)
let pinned = false;

btnPin.addEventListener('click', async () => {
  pinned = !pinned;
  const win = getCurrentWindow();
  await win.setAlwaysOnTop(pinned);
  btnPin.classList.toggle('active', pinned);
});
