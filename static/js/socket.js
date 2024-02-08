/**
 * @typedef {Object} Globals
 * @prop {WebSocket} socket
 *
 * @typedef {Window & Globals} ExtendedWindow
 */

/** @type {ExtendedWindow} */
const w = window; // for jsdoc

w.socket = new WebSocket(`ws://${location.host}/viewer`);
w.socket.addEventListener('open', () => {
  console.log('Connected to server');
})

w.socket.addEventListener('message', async (event) => {
  if (typeof event.data === 'object') {
    if (w.socket.binaryType === 'blob') {
      /** @type {ArrayBuffer} */
      var buf = await event.data.arrayBuffer();
    } else {
      var buf = event.data;
    }
  }
})
