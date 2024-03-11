/**
 * @typedef {Object} Globals
 * @prop {string} theme
 *
 * @typedef {Window & Globals} ExtendedWindow
 */

/** @type {ExtendedWindow} */
const w = window; // for jsdoc

// load theme from localstorage
let storedTheme = localStorage.getItem('theme')

// set theme to system theme if it's not set
if (!storedTheme) {
  storedTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

setTheme(storedTheme)

export function setTheme(theme) {
  w.theme = theme
  document.documentElement.setAttribute('data-theme', theme)
  localStorage.setItem('theme', theme)
}

export function getTheme() {
  return w.theme
}
