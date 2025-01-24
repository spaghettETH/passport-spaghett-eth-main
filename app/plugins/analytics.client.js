export default defineNuxtPlugin(nuxtApp => {
    window.dataLayer = window.dataLayer || [];
    function gtag() { dataLayer.push(arguments); }
    gtag('js', new Date());
    gtag('config', 'G-C7GZZYENRW');
})