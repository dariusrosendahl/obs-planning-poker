# OBS Planning Poker

A lightweight planning poker overlay for OBS Studio. Run a local server, add two browser sources to your scene, and control which card is shown during your stream or recording.

## How it works

The server exposes two pages and a WebSocket:

- **`/panel`** -- Control panel you open in your browser. Pick a card value, cycle through values, and reveal/hide the card.
- **`/card`** -- The card overlay. Add this as a Browser Source in OBS. It has a transparent background so it composites cleanly over your scene.

Both pages connect to the same WebSocket so state stays in sync. Selecting a value on the panel updates the card instantly; revealing/hiding flips the card with a CSS 3D animation.

Connects to OBS via `obs-websocket-js` (optional) for automation hooks like scene switching or source toggling. The server runs fine without OBS connected.

## Quick start

```bash
pnpm install
pnpm start
```

The server starts on **http://localhost:3000** by default.

Open `http://localhost:3000/panel` in your browser to control the card.

## OBS setup

1. Add a **Browser Source** to your scene.
2. Set the URL to `http://localhost:3000/card`.
3. Set width to **280** and height to **400**.
4. Check "Shutdown source when not visible" if you like.
5. The background is transparent -- the card floats over your scene.

## Panel controls

| Control | Action |
|---------|--------|
| Click a value | Select that card |
| `Reveal / Hide` button | Flip the card face-up or face-down |
| `Prev` / `Next` buttons | Cycle through values |
| Arrow keys | Cycle through values |
| Spacebar | Toggle reveal/hide |
| Number keys `0`-`8` | Select card by position (first nine values) |

## Card values

`0` `1` `2` `3` `5` `8` `13` `21` `?` `☕` `∞`

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | Server port |
| `OBS_WS_URL` | `ws://localhost:4455` | OBS WebSocket URL (v5) |

Override by passing environment variables inline:

```bash
PORT=9000 pnpm start
OBS_WS_URL=ws://192.168.1.10:4455 pnpm start
```

## Project structure

```
src/
  server.ts      Express + WebSocket server
  state.ts       Card state machine
  types.ts       TypeScript types and message definitions
  constants.ts   Configuration constants
public/
  card/          Browser source overlay (HTML/CSS/JS)
  panel/         Control panel UI (HTML/CSS/JS)
```

## Development

```bash
pnpm dev        # start with file watching (auto-restart on changes)
pnpm start      # start without watching
```

## Requirements

- Node.js >= 18
- pnpm

## License

MIT
