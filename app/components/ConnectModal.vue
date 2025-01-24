<template>
  <div class="connect-modal-overlay bg-blurred" id="modalOverlay" @click="clickOutsideClose($event)">

    <div v-if="isLoading">
      <Loader/>
    </div>

    <div v-if="web3Store.isConnectModal" class="connect-modal-container" :class="{
      'slide-in-bottom': web3Store.isConnectModal,
      'slide-out-bottom': web3Store.isClosingConnectModal,
    }">
      <div class="container">
        <div class="row justify-content-center" id="outerModal">
          <div class="col-12 col-md-12 col-lg-7 background-primary p-0 modal-wrapper">
            <div class="d-flex justify-content-between align-items-center p-3 modal-header">
              <div class="text-color">
                <LogoExt style="height: 25px"/>
              </div>
              <div class="d-flex ">
                <div v-if="megoWallet" @click="goBack()" class="btn-back cursor-pointer">
                  <i class="fa-solid fa-circle-chevron-left me-2"></i>
                </div>
                <div class="cursor-pointer text-color" @click="web3Store.showConnectModal()">
                  <i class="fa-solid fa-circle-xmark"></i>
                </div>
              </div>
            </div>
            <div class="d-flex justify-content-center align-items-center connect-modal-content text-center">
              <div :class="{ 'my-4': !megoWallet }">
                <div v-if="!isLoading">
                  <!-- CHOOSE WALLET CONNECTION -->
                  <div v-if="!web3Store.account && !megoWallet">
                    <!-- <button
                    class="mx-auto"
                    style="width: 200px"
                    @click="showMegoWallet()"
                  >
                    <span
                      class="text d-flex justify-content-center align-items-center"
                    >
                      <LogoMegoWallet class="me-3" /> E-mail
                    </span>
                  </button> -->

                    <button class="primary-button modal-button"
                            @click="web3Store.connect() && web3Store.showConnectModal()">
                      <div class="modal-button-content">
                        <LogoWC class="me-3 "/>
                        Wallet Connect
                      </div>
                    </button>

                    <button class="primary-button modal-button" @click="redirectToGoogleLogin">
                      <div class="modal-button-content">
                        <LogoGoogle class="me-3  "/>
                        Google Account
                      </div>
                    </button>

                    <button class="primary-button modal-button" @click="redirectToAppleLogin">
                      <div class="modal-button-content">
                        <LogoApple class="me-4"/>
                        Apple Account
                      </div>
                    </button>

                    <!-- <p class="mt-3" style="text-decoration: underline; text-underline-offset: 3px"
                    @click="showMegoWallet()">
                    I don't have a Wallet
                  </p> -->
                  </div>

                  <!-- MEGO WALLET -->
                  <div v-if="!web3Store.account && megoWallet">
                    <MegoWallet :megoStep="megoStep" @showConnectModal="showConnectModal()"/>
                  </div>

                  <!-- ACTION ON WALLET BUTTONS -->
                  <div v-if="web3Store.account">
                    <div v-if="!showExportConfirmationStep && !showPrivateKeyStep">
                      <button class="primary-button modal-button " @click="copyToClipboard(web3Store.account)">
                        <div class="modal-button-content">
                          <Clipboard class="me-3 "/>
                          {{ isCopying ? 'Copied!' : 'Copy address' }}
                        </div>
                      </button>

                      <button
                          v-if="megoWalletProvider === 'email' || megoWalletProvider === 'google' || megoWalletProvider === 'apple'"
                          class="primary-button modal-button"
                          @click="requestExportPrivateKey()">
                        <div class="modal-button-content">
                          <Lock class="me-3 "/>
                          Export key
                        </div>
                      </button>

                      <button class="primary-button modal-button" @click="editProfile()">
                        <div class="modal-button-content">
                          <Edit class="me-3 "/>
                          Edit profile
                        </div>
                      </button>

                      <button class="primary-button modal-button" @click="web3Store.disconnect()">
                        <div class="modal-button-content">
                          <Disconnect class="me-3 "/>
                          Disconnect
                        </div>
                      </button>
                    </div>

                    <div v-if="showPasswordConfirmationStep" style="text-align: center; padding: 50px 0;">
                      <p>Please enter your password to export your private key.</p>
                      <div class="custom-input-group mt-4">
                        <div v-if="password.length > 0" class="center-right" @click="togglePasswordVisibility()">
                          <i v-if="!showPassword" class="fa-solid fa-eye color-theme"></i>
                          <i v-if="showPassword" class="fa-solid fa-eye-slash color-theme"></i>
                        </div>
                        <input v-model="password" :type="showPassword ? 'text' : 'password'" autocomplete="off"
                               class="input"
                               required="" @input="validatePassword()"/>
                        <label class="user-label">Password</label>
                      </div>
                      <p v-if="!passwordIsValid" class="validation-form color-error mt-1">
                        Password is required
                      </p>

                      <button :disabled="!passwordIsValid || isWorking" class="primary-button modal-button"
                              @click="exportPrivateKeyWithEmail()">
                        Export Wallet
                      </button>

                    </div>
                    <div v-if="showExportConfirmationStep" style="text-align: center; padding: 50px 0;">
                      <p>Please enter the token received by email.</p>
                      <div class="custom-input-group mt-4">
                        <div v-if="password.length > 0" class="center-right" @click="togglePasswordVisibility()">
                          <i v-if="!showPassword" class="fa-solid fa-eye color-theme"></i>
                          <i v-if="showPassword" class="fa-solid fa-eye-slash color-theme"></i>
                        </div>
                        <input v-model="token" :type="showPassword ? 'text' : 'password'" autocomplete="off"
                               class="input"
                               required="" @input="validateToken()"/>
                        <label class="user-label">Export Token</label>
                      </div>
                      <p v-if="!passwordIsValid" class="validation-form color-error mt-1">
                        Export token is required
                      </p>
                      <button :disabled="!tokenIsValid || isWorking" class="primary-button modal-button"
                              @click="exportPrivateKeyWithToken()">
                        Reveal private key
                      </button>
                    </div>
                    <div v-if="showPrivateKeyStep" style="text-align: center; padding: 50px 0;">
                      <p>Your private key is:</p><br>
                      <pre
                          style="overflow:hidden; font-size: 11px; color:#000; background-color: #f0f0f0; padding: 10px; border-radius: 5px;">{{
                          privateKey
                        }}</pre>
                      <button class="primary-button modal-button" @click="copyToClipboard(privateKey)">
                        <div class="modal-button-content">
                          <Clipboard class="me-3 "/>
                          {{ isCopying ? 'Copied!' : 'Copy private key' }}
                        </div>
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapStores} from "pinia";
import {useWeb3Store} from "@/stores/web3Store";

import MegoWallet from "@/components/MegoWallet.vue";

import checkViewport from "@/mixins/checkViewport";

import LogoMegoWallet from "@/components/icons/LogoMegoWallet.vue";
import LogoWC from "@/components/icons/LogoWC.vue";
import LogoExt from "@/components/icons/LogoExt.vue";
import LogoApple from "@/components/icons/AppleIcon.vue";
import LogoGoogle from "@/components/icons/GoogleIcon.vue";
import Spinner from '~/components/icons/Spinner.vue'
import EditPen from "~/components/icons/EditPen.vue";
import Clipboard from "~/components/icons/Clipboard.vue";
import Disconnect from "~/components/icons/Disconnect.vue";
import Edit from "~/components/icons/Edit.vue";
import axios from "axios";
import Lock from "~/components/icons/Lock.vue";

export default {
  name: "mego-wallet",
  mixins: [checkViewport],
  data() {
    return {
      megoWallet: false,
      megoStep: "",
      isCreator: false,
      isLoading: false,
      isCopying: false,
      megoWalletUrl: import.meta.env.VITE_MEGO_WALLET_URL,
      workingMessage: "",
      token: "",
      megoWalletProvider: "",
      showPasswordConfirmationStep: false,
      showExportConfirmationStep: false,
      showPrivateKeyStep: false,
      privateKey: "",
      password: "",
      showPassword: false,
      passwordIsValid: false,
      tokenIsValid: false,
    };
  },
  components: {
    Lock,
    Edit,
    Disconnect,
    Clipboard,
    EditPen,
    MegoWallet,
    LogoMegoWallet,
    LogoWC,
    LogoExt,
    LogoApple,
    LogoGoogle,
    Spinner
  },
  computed: {
    ...mapStores(useWeb3Store),
  },
  methods: {
    togglePasswordVisibility() {
      const app = this
      app.showPassword = !app.showPassword
    },
    validatePassword() {
      const app = this
      const strongRegex = new RegExp('^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*])(?=.{6,})')
      app.passwordIsValid = strongRegex.test(app.password)
    },
    validateToken() {
      const app = this
      app.tokenIsValid = app.token.length > 0
    },

    showConnectModal() {
      const app = this;
      app.isClosing = true;
      console.log("try to close");
      setTimeout(() => {
        app.$emit("showConnectModal");
      }, 450);
    },

    showMegoWallet() {
      const app = this;
      app.megoWallet = !app.megoWallet;
    },

    goBack() {
      const app = this;
      if (app.megoWallet && app.web3Store.megoWalletStep === "") {
        app.megoWallet = false;
      }

      if (
          app.megoWallet &&
          (app.web3Store.megoWalletStep === "login" ||
              app.web3Store.megoWalletStep === "registration")
      ) {
        app.web3Store.megoWalletStep = "";
      }
    },
    clickOutsideClose(event) {
      const app = this;
      if (
          event.target.id === "modalOverlay" ||
          event.target.id === "outerModal"
      ) {
        setTimeout(() => {
          app.web3Store.showConnectModal();
        }, 200);
      }
    },

    redirectToAppleLogin() {
      const app = this;
      app.isLoading = true
      app.workingMessage = "Connecting..."
      setTimeout(function () {
        if (app.$route.params.id !== undefined) {
          window.location =
              app.megoWalletUrl + "/auth/apple?event=" + app.$route.params.id + "&origin=" + window.location.href.replace("https://", "").replace("http://", "");
        } else {
          window.location = app.megoWalletUrl + "/auth/apple" + "?origin=" + window.location.href.replace("https://", "").replace("http://", "");

        }
      }, 2500)
    },

    redirectToGoogleLogin() {
      const app = this;
      app.isLoading = true
      app.workingMessage = "Connecting..."
      setTimeout(function () {
        if (app.$route.params.id !== undefined) {
          window.location =
              app.megoWalletUrl + "/auth/google?event=" + app.$route.params.id + "&origin=" + window.location.href.replace("https://", "").replace("http://", "");

        } else {
          window.location = app.megoWalletUrl + "/auth/google" + "?origin=" + window.location.href.replace("https://", "").replace("http://", "");

        }
      }, 2500)
    },

    copyToClipboard(text) {
      const app = this;
      if (app.isCopying) {
        return;
      }
      app.isCopying = true;
      let copyText = text;
      navigator.clipboard.writeText(copyText).then(() => {
        console.log("Copied to Clipboard");
      });
      setTimeout(() => {
        app.isCopying = false;
        console.log("Copy");
      }, 1000);
    },
    editProfile() {
      this.$router.push("/profile");
      this.web3Store.showConnectModal();
    },
    requestExportPrivateKey() {
      const app = this;
      if (app.megoWalletProvider === 'email') {
        app.showPasswordConfirmationStep = true;
      } else {
        app.isWorking = true;
        app.workingMessage = "Generating link, please wait..."
        setTimeout(function () {
          if (app.megoWalletProvider === 'google') {
            window.location = app.megoWalletUrl + "/auth/google?origin=" + app.origin + "&message=EXPORT_WALLET";
          } else if (app.megoWalletProvider === 'apple') {
            window.location = app.megoWalletUrl + "/auth/apple?origin=" + app.origin + "&message=EXPORT_WALLET";
          }
        }, 500);
      }
    },
    async exportPrivateKeyWithEmail() {
      const app = this;
      app.isWorking = true;
      app.showPasswordConfirmationStep = false;
      app.workingMessage = "Sending confirmation email..."
      const megoResponse = await axios.post(import.meta.env.VITE_MEGO_WALLET_URL + '/wallets/export-request', {
        email: app.web3Store.email,
        password: app.password
      })
      console.log("EXPORT_WALLET_RESPONSE", megoResponse.data)
      app.workingMessage = megoResponse.data.message;
      setTimeout(function () {
        app.isWorking = false;
        app.showExportConfirmationStep = true;
      }, 2000);
    },
    async exportPrivateKeyWithToken() {
      const app = this;
      app.isWorking = true;
      app.showExportConfirmationStep = false;
      app.workingMessage = "Exporting private key..."
      const megoResponse = await axios.post(import.meta.env.VITE_MEGO_WALLET_URL + '/wallets/export', {
        token: app.token,
        password: app.password.length > 0 ? app.password : undefined
      })
      console.log("EXPORT_WALLET_RESPONSE", megoResponse.data)
      if (megoResponse.data.error) {
        app.workingMessage = megoResponse.data.message;
        setTimeout(function () {
          app.isWorking = false;
        }, 2000);
        return;
      }
      app.privateKey = megoResponse.data.private_keys.eth
      app.showPrivateKeyStep = true;
      app.isWorking = false;
    },
  },
  mounted() {
    this.isCopying = false;
    const app = this;
    app.isCreator = localStorage.getItem("is_creator");
    const url = new URL(window.location.href);
    app.origin = url.host
    if (app.isCreator === null) {
      app.isCreator = false;
    }
    app.megoWalletProvider = localStorage.getItem('wallet_provider')
    if (app.megoWalletProvider === null) {
      app.megoWalletProvider = 'email'
    }
    if (url.searchParams.get('exported') === 'true') {
      app.showExportConfirmationStep = true
    }
    const tokenQueryParam = url.searchParams.get('export_token')
    if (tokenQueryParam !== null) {
      app.token = tokenQueryParam
      app.exportPrivateKeyWithToken()
      url.searchParams.delete('exported')
      url.searchParams.delete('export_token')
    }
  },
  unmounted() {
    const url = new URL(window.location.href);
    window.history.replaceState({}, document.title, url);
  }
};
</script>
