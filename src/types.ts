export type CardValue =
  | '0' | '1' | '2' | '3' | '5' | '8' | '13' | '21'
  | '?' | '☕' | '∞';

export interface CardState {
  value: CardValue;
  revealed: boolean;
  index: number;
}

// Messages from client → server
export type ClientMessage =
  | { type: 'SET_CARD'; value: CardValue }
  | { type: 'NEXT_CARD' }
  | { type: 'PREV_CARD' }
  | { type: 'TOGGLE_REVEAL' }
  | { type: 'HIDE_CARD' };

// Messages from server → client
export type ServerMessage =
  | { type: 'CARD_UPDATE'; state: CardState }
  | { type: 'CONNECTED'; state: CardState };
