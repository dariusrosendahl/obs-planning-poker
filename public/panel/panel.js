const CARD_VALUES = ['0', '1', '2', '3', '5', '8', '13', '21', '?', '\u2615', '\u221E'];

const grid = document.getElementById('card-grid');
const currentValue = document.getElementById('current-value');
const btnPrev = document.getElementById('btn-prev');
const btnReveal = document.getElementById('btn-reveal');
const btnNext = document.getElementById('btn-next');

let ws;

// Build card buttons
CARD_VALUES.forEach((value) => {
  const btn = document.createElement('button');
  btn.className = 'card-btn';
  btn.textContent = value;
  btn.dataset.value = value;
  btn.addEventListener('click', () => send({ type: 'SET_CARD', value }));
  grid.appendChild(btn);
});

// Controls
btnPrev.addEventListener('click', () => send({ type: 'PREV_CARD' }));
btnNext.addEventListener('click', () => send({ type: 'NEXT_CARD' }));
btnReveal.addEventListener('click', () => send({ type: 'TOGGLE_REVEAL' }));

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
  if (e.key === 'ArrowLeft') {
    send({ type: 'PREV_CARD' });
  } else if (e.key === 'ArrowRight') {
    send({ type: 'NEXT_CARD' });
  } else if (e.key === ' ') {
    e.preventDefault();
    send({ type: 'TOGGLE_REVEAL' });
  } else if (e.key >= '0' && e.key <= '8') {
    const index = parseInt(e.key, 10);
    if (index < CARD_VALUES.length) {
      send({ type: 'SET_CARD', value: CARD_VALUES[index] });
    }
  }
});

// WebSocket
function connect() {
  const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
  ws = new WebSocket(`${protocol}//${location.host}`);

  ws.addEventListener('message', (event) => {
    const msg = JSON.parse(event.data);
    if (msg.type === 'CONNECTED' || msg.type === 'CARD_UPDATE') {
      updatePanel(msg.state);
    }
  });

  ws.addEventListener('close', () => {
    setTimeout(connect, 2000);
  });
}

function send(msg) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(msg));
  }
}

function updatePanel(state) {
  currentValue.textContent = state.value;

  document.querySelectorAll('.card-btn').forEach((btn) => {
    btn.classList.toggle('active', btn.dataset.value === state.value);
  });
}

connect();
