import { writable } from "svelte/store";

export const availableThemes = [
  'light',
  'dark',
  'cupcake',
  'bumblebee',
  'emerald',
  'corporate',
  'synthwave',
  'retro',
  'cyberpunk',
  'valentine',
  'halloween',
  'garden',
  'forest',
  'aqua',
  'lofi',
  'pastel',
  'fantasy',
  'wireframe',
  'black',
  'luxury',
  'dracula',
  'cmyk',
  'autumn',
  'business',
  'acid',
  'lemonade',
  'night',
  'coffee',
  'winter'
] as const;

export type Options = {
  theme: typeof availableThemes[number],
  followBottom: boolean,
}

const loadOptions = (): Options => {
  const theme = localStorage.getItem('markdown-preview-server__options__theme');
  const followBottom = localStorage.getItem('markdown-preview-server__options__followBottom');

  return {
    theme: theme !== null ? theme as typeof availableThemes[number] : (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'),
    followBottom: followBottom === 'true'
  }
}

export const options = writable<Options>(loadOptions());

options.subscribe((value) => {
  console.log('saving settings')
  localStorage.setItem('markdown-preview-server__options__theme', String(value.theme));
  localStorage.setItem('markdown-preview-server__options__followBottom', String(value.followBottom.toString()));
})
