/** @type {import('@sveltejs/kit').Config} */
import sveltePreprocess from 'svelte-preprocess'
import adapter from '@sveltejs/adapter-static'

export default {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: sveltePreprocess(),
  kit: {
    adapter: adapter({
      fallback: 'index.html',
      pages: 'dist'
    }),
    files: {
      template: 'src/index.html'
    },
    prerender: {
      default: true
    }
  }
}
