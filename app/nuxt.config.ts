import {NodeGlobalsPolyfillPlugin} from '@esbuild-plugins/node-globals-polyfill'
import {NodeModulesPolyfillPlugin} from '@esbuild-plugins/node-modules-polyfill'
import nodePolyfills from 'rollup-plugin-polyfill-node'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    runtimeConfig: {
        public: {
            VITE_API_URL: process.env.VITE_API_URL,
            VITE_DEBUG: process.env.VITE_DEBUG,
            VITE_BLOCKCHAIN: process.env.VITE_BLOCKCHAIN,
            VITE_BLOCKCHAIN_DEFAULT_PROCESSOR: process.env.VITE_BLOCKCHAIN_DEFAULT_PROCESSOR,
            VITE_MEGO_WALLET_URL: process.env.VITE_MEGO_WALLET_URL,
            VITE_WALLETCONNECT: process.env.VITE_WALLETCONNECT
        }
    },
    modules: [
        'nuxt-gtag',
        '@pinia/nuxt',
    ],
    // gtag: {
    //   id: 'G-C7GZZYENRW'
    // },
    css: [
        '@/fonts/stylesheet.css',
        '@/styles/responsive.scss',
        '@/styles/animation.scss',
        '@/styles/main.scss',
    ],
    app: {
        head: {
            meta: [
                {charset: 'utf-8'},
                {name: 'viewport', content: 'width=device-width, initial-scale=1'},
                {name: 'keywords', content: ''},
                {name: 'description', content: ""},
                {property: 'og:title', content: ''},
                {property: 'og:url', content: ''},
                {property: 'og:image', content: ''},
                {property: 'og:description', content: ""}
            ],
            link: [
                {rel: 'stylesheet', href: 'https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css'},
                {rel: 'stylesheet', href: 'https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css'},
                {rel: 'manifest', href: '/manifest.webmanifest'},
            ],
            script: [
                {src: 'https://js.stripe.com/v3/'},
                {src: 'https://www.googletagmanager.com/gtag/js?id=G-C7GZZYENRW'}
            ]
        },
    },
    vite: {
        logLevel: 'info',
        // plugins: [eslintPlugin()],
        optimizeDeps: {
            esbuildOptions: {
                define: {
                    global: 'globalThis'  // fix nuxt3 global
                },
                plugins: [
                    NodeGlobalsPolyfillPlugin({
                        process: true,  // fix nuxt3 process
                        buffer: true
                    }),
                    NodeModulesPolyfillPlugin()
                ]
            }
        },
        rollupOptions: {
            plugins: [
                nodePolyfills()
            ],
        }
    }
})
