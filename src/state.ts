import type { CardState, CardValue } from './types.js';
import { CARD_VALUES } from './constants.js';

let state: CardState = {
  value: CARD_VALUES[0],
  revealed: false,
  index: 0,
};

export function getState(): CardState {
  return { ...state };
}

export function setCard(value: CardValue): CardState {
  const index = CARD_VALUES.indexOf(value);
  if (index === -1) return getState();
  state = { value, revealed: state.revealed, index };
  return getState();
}

export function nextCard(): CardState {
  const index = (state.index + 1) % CARD_VALUES.length;
  state = { value: CARD_VALUES[index], revealed: state.revealed, index };
  return getState();
}

export function prevCard(): CardState {
  const index = (state.index - 1 + CARD_VALUES.length) % CARD_VALUES.length;
  state = { value: CARD_VALUES[index], revealed: state.revealed, index };
  return getState();
}

export function toggleReveal(): CardState {
  state = { ...state, revealed: !state.revealed };
  return getState();
}

export function hideCard(): CardState {
  state = { ...state, revealed: false };
  return getState();
}
