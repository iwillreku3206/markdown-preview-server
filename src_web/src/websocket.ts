import { writable } from "svelte/store"

const ws = new WebSocket('ws://127.0.0.1:8081')

export function send(message: Blob | string | ArrayBuffer | ArrayBufferView) {
  ws.send(message)
}
export const messageStore = writable<Blob>()

ws.onopen = () => {
  console.log('connected')
}

ws.onmessage = (event) => {
  messageStore.set(event.data)
}

