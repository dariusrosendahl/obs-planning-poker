const CARD_VALUES = ['0', '1', '2', '3', '5', '8', '13', '21', '?', '\u2615', '\u221E'];

const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;

const grid = document.getElementById('card-grid');
const currentValue = document.getElementById('current-value');
const btnPrev = document.getElementById('btn-prev');
const btnReveal = document.getElementById('btn-reveal');
const btnNext = document.getElementById('btn-next');
const btnPin = document.getElementById('btn-pin');

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
}

// Load initial state
sendCommand('get_state');

// Always on top (native window pinning)
let pinned = false;

btnPin.addEventListener('click', async () => {
  pinned = !pinned;
  const win = getCurrentWindow();
  await win.setAlwaysOnTop(pinned);
  btnPin.classList.toggle('active', pinned);
});
