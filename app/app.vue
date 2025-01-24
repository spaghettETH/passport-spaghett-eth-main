<template>
  <div :class="{
    'd-flex justify-content-center flex-column':
      (($route.name === 'index' && web3Store.showBigQr) || $route.name !== 'index') || !web3Store.account,
    'login-arts': $route.name === 'index' && !web3Store.account ,
  }" style="min-height: 100dvh; position: relative; border-radius: 0px !important">
    <div class="container" :class="{ 'pt-header': $route.name === 'index' && !web3Store.showBigQr }">
      <router-link to="/">
        <div class="logo-overlay"></div>
      </router-link>
      <NuxtPage/>
      <div v-if="web3Store.isConnectModal">
        <ConnectModal/>
      </div>
      <div v-if="web3Store.isAlert">
        <Alert/>
      </div>
    </div>
    <!-- FOOTER -->
    <!-- <div v-if="!isDesktop && !isLargeDesktop" class="gap-footer"></div> -->
    <div class="background-primary" style="position:fixed;width: 100%; padding:10px 0; z-index: 99; bottom:0; left: 0;">
      <div class="container">
        <div class="row justify-content-center">
          <div class="col-12 col-md-12 col-lg-8">
            <FooterApp/>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapStores} from "pinia";
import {useWeb3Store} from "@/stores/web3Store";

import Alert from "@/components/Alert.vue";
import ConnectModal from "./components/ConnectModal.vue";

import FooterApp from "@/components/FooterApp.vue";

export default {
  name: "app",
  data() {
    return {
      isDesktop: false,
      isLargeDesktop: false,
      isLoading: true,
    };
  },
  components: {
    FooterApp,
    ConnectModal,
    Alert,
  },
  computed: {
    ...mapStores(useWeb3Store),
  },
  mounted() {
    const app = this;
    if (window.location.href.indexOf("loggedAs") !== -1) {
      const raw = window.location.href.split("?")[1].split("&");
      const params = {};
      for (let k in raw) {
        params[raw[k].split("=")[0]] = raw[k].split("=")[1];
      }
      console.log("QUERY_PARAMS", params);
      localStorage.setItem("wallet_type", "mego");
      localStorage.setItem("wallet_address", params.loggedAs);
      localStorage.setItem("wallet_provider", params.provider);
      const url = new URL(window.location.href);
      url.searchParams.delete("loggedAs");
      url.searchParams.delete("provider");
      window.location.href = "/";
    }
    if (window.location.href.indexOf("exported=true") !== -1) {
      console.log("EXPORT_WALLET_SUCCESS");
      app.web3Store.showConnectModal(true);
    }
    app.web3Store.instantiateWeb3();
  },
  beforeCreate() {
    setTimeout(() => {
      this.privacyPolicy();
    }, 1000);
  },
  methods: {
    privacyPolicy() {
      if (process.client) {
        const scriptIubenda = document.createElement("script");
        scriptIubenda.type = "text/javascript";
        scriptIubenda.innerHTML = `
        var _iub = _iub || [];
        _iub.csConfiguration = {
          askConsentAtCookiePolicyUpdate: true,
          countryDetection: true,
          enableLgpd: true,
          enableUspr: true,
          lgpdAppliesGlobally: false,
          perPurposeConsent: true,
          siteId: 3118995,
          cookiePolicyId: 84994680,
          lang: "en-GB",
          banner: {
            acceptButtonCaptionColor: "#000000",
            acceptButtonColor: "#8BFBD9",
            acceptButtonDisplay: true,
            closeButtonDisplay: false,
            customizeButtonCaptionColor: "#000000",
            customizeButtonColor: "#8BFBD9",
            customizeButtonDisplay: true,
            explicitWithdrawal: true,
            listPurposes: true,
            position: "top",
            rejectButtonCaptionColor: "#000000",
            rejectButtonColor: "#8BFBD9",
            rejectButtonDisplay: true,
            showPurposesToggles: true,
            showBannerForUS: false,
          },
        };
      `;
        document.head.append(scriptIubenda);
        const scriptIubenda2 = document.createElement("script");
        scriptIubenda2.type = "text/javascript";
        scriptIubenda2.src = "//cdn.iubenda.com/cs/gpp/stub.js";
        document.head.append(scriptIubenda2);
        const scriptIubenda3 = document.createElement("script");
        scriptIubenda3.type = "text/javascript";
        scriptIubenda3.src = "//cdn.iubenda.com/cs/iubenda_cs.js";
        scriptIubenda3.charset = "UTF-8";
        document.head.append(scriptIubenda3);
      }
    },
  },
};
</script>
