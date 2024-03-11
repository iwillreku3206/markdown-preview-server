const opcodeHandlers = {
  0x0100: setText,
}

function setText(buf) {
  /** @type {HTMLDivElement} */
  const content = document.getElementById('document')
  content.innerHTML = new TextDecoder().decode(buf.slice(2))
}

window.addEventListener('message', (event) => {
  // this page is only supposed to receive messages from the iframe on the
  // main page, so it is assumed that the message is a Uint8Array

  /** @type {Uint8Array} */
  const buf = event.data
  const opcode = buf[0] << 8 | buf[1]
  console.log(opcode)
  if (opcode in opcodeHandlers) {
    opcodeHandlers[opcode](buf)
  }
})
