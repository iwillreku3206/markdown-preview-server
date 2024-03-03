/** @type {import('tailwindcss').Config} */
const config = {
  content: ["./{templates,src,static}/**/*.{html,js}"],
  theme: {
    colors: {
      'background-1': 'var(--background-1)',
      'background-2': 'var(--background-2)',
      'background-3': 'var(--background-3)',
      'text-1': 'var(--text-1)',
      'text-2': 'var(--text-2)',
      'text-3': 'var(--text-3)',
      'select-tint': 'var(--select-tint)',
      'not-selected-tint': 'var(--not-selected-tint)',
    },
    extend: {},
  },
  plugins: []
}

module.exports = config

