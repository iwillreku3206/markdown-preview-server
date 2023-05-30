import daisyui from 'daisyui';

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,svelte,ts,js}"],
  theme: {
    extend: {
      maxWidth: {
        'container-small': '100vw',
        'container': '1024px'
      }
    },
  },
  plugins: [daisyui],
}

