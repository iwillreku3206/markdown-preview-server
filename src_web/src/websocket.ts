import { writable } from "svelte/store"

const ws = new WebSocket('ws://127.0.0.1:8081')

export const messageStore = writable<Blob>()

ws.onopen = () => {
  console.log('connected')
}

ws.onmessage = (event) => {
  //  messageStore.set(new Blob())
  messageStore.set(event.data)
}
