const card = document.getElementById('card');
const centerValue = card.querySelector('.center-value');
const topLeft = card.querySelector('.corner.top-left');
const bottomRight = card.querySelector('.corner.bottom-right');

let ws;

function connect() {
  const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
  ws = new WebSocket(`${protocol}//${location.host}`);

  ws.addEventListener('message', (event) => {
    const msg = JSON.parse(event.data);

    if (msg.type === 'CONNECTED' || msg.type === 'CARD_UPDATE') {
      updateCard(msg.state);
    }
  });

  ws.addEventListener('close', () => {
    setTimeout(connect, 2000);
  });
}

function updateCard(state) {
  centerValue.textContent = state.value;
  topLeft.textContent = state.value;
  bottomRight.textContent = state.value;

  if (state.revealed) {
    card.classList.remove('flipped');
  } else {
    card.classList.add('flipped');
  }
}

connect();
