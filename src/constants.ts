import type { CardValue } from './types.js';

export const CARD_VALUES: CardValue[] = [
  '0', '1', '2', '3', '5', '8', '13', '21', '?', '☕', '∞',
];

export const SERVER_PORT = Number(process.env.PORT) || 3000;
export const OBS_WS_URL = process.env.OBS_WS_URL || 'ws://localhost:4455';
