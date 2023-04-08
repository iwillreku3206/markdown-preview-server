import { writable } from "svelte/store"

const ws = new WebSocket('ws://127.0.0.1:8081')

const messageStore = writable('')

ws.onopen = () => {
  console.log('connected')
}

ws.onmessage = (event) => {
  messageStore.set('')
  messageStore.set(event.data)
  console.log(event.data)
}

export const subscribe = messageStore.subscribe
