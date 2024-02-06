// import path from "path";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    "@nuxtjs/tailwindcss",
    "@nuxtjs/color-mode",
    "nuxt-primevue",
    "@pinia/nuxt",
    "@nuxtjs/eslint-module",
    "@vueuse/nuxt",
    // "nuxt-icon",
    "@nuxt/test-utils/module",
  ],
  eslint: {
    lintOnStart: false,
  },
  primevue: {
    options: { unstyled: true, ripple: true },
    importPT: { as: "Wind", from: "~/presets/wind/" }, // import and apply preset
    components: {
      exclude: ["Editor", "Chart"],
    },
  },
  pinia: {
    storesDirs: ["./stores/**"],
  },
  colorMode: {
    classSuffix: "",
  },
  tailwindcss: {
    cssPath: "~/assets/css/tailwind.css",
    editorSupport: true,
    config: {},
  },
  css: [
    "~/assets/css/inter.css",
    "~/assets/css/base.css",
    "primeicons/primeicons.css",
    // "primevue/resources/themes/aura-light-teal/theme.css",
    // "primevue/resources/themes/aura-dark-teal/theme.css",
  ],
  devtools: { enabled: true },
  // Enable SSG
  ssr: false,
  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://tauri.app/2/reference/environment-variables/
    envPrefix: ["VITE_"],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
      hmr: {
        // Use websocket for mobile hot reloading
        protocol: "ws",
        // Make sure it's available on the network
        host: "0.0.0.0",
        // Use a specific port for hmr
        port: 5183,
      },
    },
  },
});
