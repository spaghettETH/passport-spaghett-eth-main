<template>
  <div class="container" :class="{ 'pb-4': !isWorking, 'pb-5': isWorking }">
    <div class="row justify-content-center">
      <div class="col-12">
        <!-- WORKING MESSAGE -->
        <div class="
                          working-container
                          d-flex
                          justify-content-center
                          align-items-center
                          flex-column
                          pt-5
                        " v-if="isWorking">
          <Spinner />
          <p class="mt-2">{{ workingMessage }}</p>
        </div>

        <!-- CHOOSE AN ACTION -->
        <div v-if="web3Store.megoWalletStep === '' && !isWorking && signMessageStep === ''" class="text-center">
          <h3>Choose an action</h3>
          <p class="mt-2">
            Sign in or sign up using Mego Wallet
            <a href="" target="_blank"><i class="fa-solid fa-circle-question ms-1"></i></a>
          </p>
          <button class="mx-auto mt-4" style="width: 200px" @click="web3Store.megoWalletStep = 'login'">
            <span class="text"> Login</span>
          </button>
          <button class="mx-auto mt-4" style="width: 200px" @click="web3Store.megoWalletStep = 'registration'">
            <span class="text">Sign Up</span>
          </button>
        </div>

        <!-- CREATION STEP -->
        <div v-if="web3Store.megoWalletStep === 'registration' && !isWorking && signMessageStep === ''"
          class="text-center">
          <h3>Create or restore a wallet</h3>
          <p class="mt-2">
            Type your mail and choose a password to create <br />
            your wallet.
          </p>

          <div class="mt-4 px-5">
            <!-- E-MAIL INPUT -->
            <div class="custom-input-group">
              <input required="" type="text" autocomplete="off" class="input" @input="checkEmail()" v-model="email" />
              <label class="user-label">E-mail address</label>
            </div>
            <p v-if="!emailIsValid && email.length > 5" class="validation-form color-error mt-1">
              make sure you have entered the email correctly
            </p>

            <!-- PASSWORD INPUT -->
            <div>
              <div class="custom-input-group mt-4">
                <div v-if="password.length > 0" class="center-right" @click="togglePasswordVisibility()">
                  <i v-if="!showPassword" class="fa-solid fa-eye color-theme"></i>
                  <i v-if="showPassword" class="fa-solid fa-eye-slash color-theme"></i>
                </div>
                <input required="" :type="showPassword ? 'text' : 'password'" autocomplete="off" class="input"
                  v-model="password" @input="validatePassword()" />
                <label class="user-label">Password</label>
              </div>
              <p v-if="!passwordIsValid && password.length > 3" class="validation-form color-error mt-1">
                6 characters, an uppercase letter, a number, and a special character required.
              </p>
            </div>

            <!-- CONFIRM PASSWORD INPUT -->
            <div>
              <div class="custom-input-group mt-4">
                <div v-if="confirmPassword.length > 0" class="center-right" @click="toggleConfirmPasswordVisibility()">
                  <i v-if="!showConfirmPassword" class="fa-solid fa-eye color-theme"></i>
                  <i v-if="showConfirmPassword" class="fa-solid fa-eye-slash color-theme"></i>
                </div>
                <input required="" :type="showConfirmPassword ? 'text' : 'password'" autocomplete="off" class="input"
                  v-model="confirmPassword" @input="checkPasswordMatch" />
                <label class="user-label">Repeat Password</label>
              </div>
              <p v-if="!passwordMatch && confirmPassword.length >= 6" class="validation-form color-error mt-1">
                Passwords do not match
              </p>
            </div>

            <button class="mx-auto mt-4" :disabled="!passwordIsValid || !emailIsValid || !passwordMatch || isWorking"
              @click="createNewWallet()">
              <span class="text">Create Wallet</span>
            </button>
          </div>
        </div>

        <!-- LOGIN/CHECK WALLET STEP -->
        <div v-if="web3Store.megoWalletStep === 'login' && !isWorking && signMessageStep === ''" class="text-center">
          <h3>Login</h3>
          <p class="mt-2">Type your mail and and password to login</p>

          <div class="mt-4 px-5">
            <!-- E-MAIL INPUT -->
            <div class="custom-input-group">
              <input required="" type="text" autocomplete="off" class="input" @input="checkEmail()" v-model="email" />
              <label class="user-label">E-mail address</label>
            </div>
            <p v-if="!emailIsValid && email.length > 5" class="validation-form color-error mt-1">
              make sure you have entered the email correctly
            </p>

            <!-- PASSWORD INPUT -->
            <div>
              <div class="custom-input-group mt-4">
                <div v-if="password.length > 0" class="center-right" @click="togglePasswordVisibility()">
                  <i v-if="!showPassword" class="fa-solid fa-eye color-theme"></i>
                  <i v-if="showPassword" class="fa-solid fa-eye-slash color-theme"></i>
                </div>
                <input required="" :type="showPassword ? 'text' : 'password'" autocomplete="off" class="input"
                  v-model="password" />
                <label class="user-label">Password</label>
              </div>
            </div>

            <!-- CONFIRM PASSWORD INPUT -->
            <button class="mx-auto mt-4" :disabled="password.length === 0 || !emailIsValid || isWorking"
              @click="checkWallet()">
              <span class="text">Login</span>
            </button>
          </div>
        </div>

        <!-- SIGN MESSAGE STEP -->
        <div v-if="web3Store.megoWalletStep !== 'login' &&
          web3Store.megoWalletStep !== 'registration' &&
          !isWorking &&
          signMessageStep === 'mego'
        " class="text-center">
          <h3>Sign</h3>
          <div v-if="megoWalletProvider === 'email' || megoWalletProvider === null">
            <p class="mt-2">Type your password to sign message</p>
            <div class="mt-4 px-5">
              <!-- PASSWORD INPUT -->
              <div>
                <div class="custom-input-group mt-4">
                  <div v-if="password.length > 0" class="center-right" @click="togglePasswordVisibility()">
                    <i v-if="!showPassword" class="fa-solid fa-eye color-theme"></i>
                    <i v-if="showPassword" class="fa-solid fa-eye-slash color-theme"></i>
                  </div>
                  <input required="" :type="showPassword ? 'text' : 'password'" autocomplete="off" class="input"
                    v-model="password" />
                  <label class="user-label">Password</label>
                </div>
              </div>

              <button style="width: 100%" class="mx-auto mt-4" :disabled="password.length < 5" @click="signMessage()">
                <span class="text">Confirm</span>
              </button>
            </div>
          </div>
          <!-- Sign with Apple Account-->
          <div v-if="megoWalletProvider === 'apple'">
            <p class="mt-2">Sign message with Apple</p>
            <div class="mt-4 px-3">
              <button style="width: 100%" class="mx-auto mt-4" @click="signMessageWithApple()">
                <span class="text">Sign with Apple</span>
              </button>
            </div>
          </div>
          <!-- Sign with Google Account-->
          <div v-if="megoWalletProvider === 'google'">
            <p class="mt-2">Sign message with Google</p>
            <div class="mt-4 px-3">
              <button style="width: 100%" class="mx-auto mt-4" @click="signMessageWithGoogle()">
                <span class="text">Sign with Google</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

import { mapStores } from 'pinia'
import { useWeb3Store } from '@/stores/web3Store'

import Spinner from '@/components/icons/Spinner.vue'

export default {
  name: 'mego-wallet',
  props: ['isSignMessage', 'tokenId'],
  data() {
    return {
      email: '',
      password: '',
      confirmPassword: '',
      megoWalletProvider: '',
      megoWalletUrl: import.meta.env.VITE_MEGO_WALLET_URL,

      // WALLET CREATION STEPS
      signMessageStep: this.isSignMessage,

      // INPUT CHECKER
      showPassword: false,
      showConfirmPassword: false,
      passwordIsValid: false,
      emailIsValid: false,
      passwordMatch: false,

      // WORKING STATUS
      isWorking: false,
      workingMessage: '',

      // SIGN MESSAGES
      claimSignature: ''
    }
  },
  computed: {
    ...mapStores(useWeb3Store)
  },
  components: {
    Spinner
  },
  mounted() {
    const app = this
    // CHECK IF SIGNMESSAGE VAR EXISTS
    if (app.signMessageStep === undefined || app.signMessageStep === null) {
      app.signMessageStep = ''
    }
    if (app.web3Store.isDebug) {
      console.log('💾 MegoWallet.vue ==> signMessageStep is:', app.signMessageStep)
    }
    app.megoWalletProvider = localStorage.getItem('wallet_provider')
    if (app.megoWalletProvider === null) {
      app.megoWalletProvider = 'email'
    }
  },
  methods: {
    // CHECKERS FORM
    validatePassword() {
      const app = this
      const strongRegex = new RegExp('^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*])(?=.{6,})')
      app.passwordIsValid = strongRegex.test(app.password)
    },

    checkEmail() {
      const app = this
      var emailRegex =
        /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
      app.emailIsValid = emailRegex.test(app.email)
    },

    checkPasswordMatch() {
      const app = this
      app.passwordMatch = app.password === app.confirmPassword
      if (app.web3Store.isDebug) {
        console.log('✅ password match?', app.passwordMatch)
      }
    },

    togglePasswordVisibility() {
      const app = this
      app.showPassword = !app.showPassword
    },

    toggleConfirmPasswordVisibility() {
      const app = this
      app.showConfirmPassword = !app.showConfirmPassword
    },

    // CREATE WALLET
    async createNewWallet() {
      const app = this
      if (!app.isWorking) {
        app.isWorking = true
        if (app.web3Store.isDebug) {
          console.log('🚀  Creating new mego wallet')
        }
        // LOWERCASE EMAIL
        app.email = app.email.toLowerCase()
        app.workingMessage = 'Creating new account..'
        const create = await axios.post(import.meta.env.VITE_MEGO_WALLET_URL + '/wallets/email/new', {
          email: app.email,
          password: app.password
        })
        if (!create.data.error) {
          app.workingMessage = 'Verifying new account..'
          const check = await axios.post(import.meta.env.VITE_MEGO_WALLET_URL + '/wallets/check', {
            email: app.email,
            password: app.password
          })
          if (check.data.error === false) {
            app.isWorking = false
            app.workingMessage = ''
            app.web3Store.megoWalletStep = 'login'
            app.email = ''
            app.password = ''
          } else {
            let alertMessage = check.data.message
            app.web3Store.showAlert(alertMessage)
            app.isWorking = false
            app.workingMessage = ''
          }
        } else {
          alert(create.data.message)
          app.isWorking = false
          app.workingMessage = ''
        }
      }
    },

    // CHECK WALLET
    async checkWallet() {
      const app = this
      if (app.web3Store.isDebug) {
        console.log('🚀 Login MEGO wallet..')
      }
      app.email = app.email.toLowerCase()
      app.isWorking = true
      app.workingMessage = 'Waiting for wallet service..'
      const check = await axios.post(import.meta.env.VITE_MEGO_WALLET_URL + '/wallets/check', {
        email: app.email,
        password: app.password
      })
      if (app.web3Store.isDebug) {
        console.log('🚀 check wallet')
      }
      if (check.data.error === false) {
        app.workingMessage = 'Wallet restored successfully..'
        app.account = check.data.addresses.eth
        app.isWorking = false
        app.web3Store.walletType = 'mego'
        app.web3Store.megoWalletStep = ''
        try {
          localStorage.setItem('wallet_type', 'mego')
          localStorage.setItem('wallet_provider', 'email')
          localStorage.setItem('wallet_address', check.data.addresses.eth)
          localStorage.setItem('wallet_email', app.email)
          app.web3Store.showConnectModal()
          app.web3Store.cachedConnection()
        } catch (e) {
          console.log('ERROR WHILE SETTING LOCAL STORAGE')
          console.log(e.message)
          console.log('-----------------')
        }
      } else {
        if (check.data.reason === 'NOT_FOUND') {
          // app.step = 'registration'
          app.isWorking = false
          app.workingMessage = ''
          let alertMessage = 'User not found'
          app.web3Store.showAlert(alertMessage)
        } else if (check.data.reason === 'WRONG_PASSWORD') {
          // alert('Wrong password, please retry!')
          app.isWorking = false
          app.workingMessage = ''
          let alertMessage = 'Wrong password, please retry!'
          app.web3Store.showAlert(alertMessage)
        }
      }
    },

    // SIGN MESSAGE
    async signMessage() {
      const app = this
      if (!app.isWorking) {
        app.isWorking = true
        app.workingMessage = 'Waiting for signing service..'
        try {
          const walletEmail = app.web3Store.email
          const signature = await axios.post(
            import.meta.env.VITE_MEGO_WALLET_URL + '/transactions/sign',
            {
              email: walletEmail,
              password: app.password,
              message: 'Claiming token ' + app.tokenId + '.'
            }
          )
          app.isWorking = false
          if (!signature.data.error) {
            app.claimSignature = signature.data.signature
            app.isWorking = true
            app.signMessageStep = ''
            app.web3Store.megoWalletStep = ''
            app.$emit('prepareClaimMego', app.claimSignature)
            // app.claim()
          } else {
            app.web3Store.showAlert(signature.data.message)
          }
        } catch (e) {
          // app.isClaimed = false
          app.web3Store.showAlert(e.message)
          setTimeout(function () {
            app.web3Store.isAlert = false
          }, 3000)
          setTimeout(function () {
            app.isWorking = false
          }, 3000)
        }
      }
    },

    signMessageWithGoogle() {
      const app = this
      localStorage.setItem("claiming_token", app.tokenId)
      window.location = app.megoWalletUrl + "/auth/google?event=" + app.$route.params.id + "&message=" + 'Claiming token ' + app.tokenId + '.'
    },

    signMessageWithApple() {
      const app = this
      localStorage.setItem("claiming_token", app.tokenId)
      window.location = app.megoWalletUrl + "/auth/apple?event=" + app.$route.params.id + "&message=" + 'Claiming token ' + app.tokenId + '.'
    }
  }
}
</script>
