import daisyui from 'daisyui';

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,svelte,ts,js}"],
  theme: {
    extend: {
      maxWidth: {
        'container': '1024px'
      }
    },
  },
  plugins: [daisyui],
}

