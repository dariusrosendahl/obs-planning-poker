import express from 'express';
import { createServer } from 'node:http';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { WebSocketServer, WebSocket } from 'ws';
import OBSWebSocket from 'obs-websocket-js';
import { SERVER_PORT, OBS_WS_URL } from './constants.js';
import { getState, setCard, nextCard, prevCard, toggleReveal, hideCard } from './state.js';
import type { ClientMessage, ServerMessage } from './types.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const publicDir = path.join(__dirname, '..', 'public');

// Express
const app = express();
app.use('/panel', express.static(path.join(publicDir, 'panel')));
app.use('/card', express.static(path.join(publicDir, 'card')));

const server = createServer(app);

// WebSocket server
const wss = new WebSocketServer({ server });

function broadcast(msg: ServerMessage) {
  const data = JSON.stringify(msg);
  for (const client of wss.clients) {
    if (client.readyState === WebSocket.OPEN) {
      client.send(data);
    }
  }
}

wss.on('connection', (ws) => {
  const connected: ServerMessage = { type: 'CONNECTED', state: getState() };
  ws.send(JSON.stringify(connected));

  ws.on('message', (raw) => {
    let msg: ClientMessage;
    try {
      msg = JSON.parse(String(raw));
    } catch {
      return;
    }

    let newState;
    switch (msg.type) {
      case 'SET_CARD':
        newState = setCard(msg.value);
        break;
      case 'NEXT_CARD':
        newState = nextCard();
        break;
      case 'PREV_CARD':
        newState = prevCard();
        break;
      case 'TOGGLE_REVEAL':
        newState = toggleReveal();
        break;
      case 'HIDE_CARD':
        newState = hideCard();
        break;
      default:
        return;
    }

    broadcast({ type: 'CARD_UPDATE', state: newState });
  });
});

// OBS WebSocket (optional, auto-reconnect)
const obs = new OBSWebSocket();
let obsConnected = false;

async function connectOBS() {
  if (obsConnected) return;
  try {
    await obs.connect(OBS_WS_URL);
    obsConnected = true;
    console.log(`[OBS] Connected to ${OBS_WS_URL}`);

    obs.once('ConnectionClosed', () => {
      obsConnected = false;
      console.log('[OBS] Disconnected, retrying in 10s...');
      setTimeout(connectOBS, 10_000);
    });
  } catch {
    console.log('[OBS] Not connected — start OBS and restart the server to enable integration');
  }
}

// Start
server.listen(SERVER_PORT, () => {
  console.log(`Planning Poker server running on http://localhost:${SERVER_PORT}`);
  console.log(`  Panel: http://localhost:${SERVER_PORT}/panel`);
  console.log(`  Card:  http://localhost:${SERVER_PORT}/card`);
  connectOBS();
});
