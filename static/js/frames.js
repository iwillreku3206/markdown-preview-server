/**
 * @callback FrameHandler 
 * @param {ArrayBuffer} buf
 */

/** @type {object.<number, FrameHandler>} */
const opcodeHandlers = {
  0x0001: ping,
  0x0002: pong,
  0x0100: setText,
  0x0101: setDocumentTitle,
  0x0102: setFilePath,
  0x0103: setCursorPosition, // UNUSED
  0xffff: closeConnection,
}

/** @type {ExtendedWindow} */
const w = window;

/**
 *  @param {ArrayBuffer} buf 
 */
export function onMessage(buf) {
  buf = new Uint8Array(buf)
  const opcode = buf[0] << 8 | buf[1]
  console.log(opcode)
  if (opcode in opcodeHandlers) {
    opcodeHandlers[opcode](buf)
  }
}

function ping() {
  w.socket.send(new Uint16Array([0x0002]).buffer)
}

function pong() { /* STUF */ }

/**
 * @param {ArrayBuffer} buf 
 */
function setText(buf) {
  document.getElementById('document').innerHTML = new TextDecoder().decode(buf.slice(2))
}

/**
 * @param {ArrayBuffer} buf 
 */
function setDocumentTitle(buf) {

}

/**
 * @param {ArrayBuffer} buf 
 */
function setFilePath(buf) {

}

/**
 * @param {ArrayBuffer} buf 
 */
function setCursorPosition(buf) {

}

function closeConnection() {

}
