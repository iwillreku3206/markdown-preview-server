import { writable } from "svelte/store"

export default function connect() {
  const ws = new WebSocket('ws://127.0.0.1:8081')

  const messageStore = writable<string | Blob>('')

  ws.onopen = () => {
    console.log('connected')
  }

  ws.onmessage = (event) => {
    messageStore.set('')
    messageStore.set(event.data)
  }
  return messageStore.subscribe
}
