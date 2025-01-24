<script setup>
import Eyes from "~/components/icons/Eyes.vue";

useSeoMeta({
  title: 'SpaghettETH Passport',
  description: "SpaghettETH Official Passport",
  ogTitle: 'SpaghettETH Passport',
  ogDescription: "SpaghettETH Official Passport",
  ogImage: "https://app.mego.tickets/cover.png",
  twitterCard: 'summary_large_image',
  twitterImage: "https://app.mego.tickets/cover.png",
  twitterDescription: "SpaghettETH Official Passport",
  twitterTitle: 'SpaghettETH Passport',
  twitterUrl: 'https://app.mego.tickets/',
  twitterDomain: 'mego.tickets'
})
</script>
<template>
  <div class="blurred">
    <div v-if="isLoading">
      <Loader />
    </div>

    <div v-if="!web3Store.account && !isLoading">
      <MainContent />
    </div>

    <div v-if="!isLoading && web3Store.account">
      <div class="d-flex flex-column justify-content-center align-items-center">

        <div v-if="!collecting">
          <div v-if="checkpoint" class="d-flex flex-column justify-content-center align-items-center">
            <CheckpointDetails :checkpoint="checkpoint" button-label="Collect" @onClick="collectSignature" />
          </div>
        </div>

        <div v-else>
          <div v-if="qrSuccess" class="text-color-pink text-center mb-4 px-4 alert-card" style="font-size: 90px;">
            <h1 class="text-bold font-2xl">Congrats!</h1>
            <p class="text-semibold font-xl">{{ qrResponse }}</p>
            <button :class="{ 'mx-auto': !isDesktop && !isLargeDesktop }"
              class="mt-3 primary-button text-bold uppercase font-lg " @click="goBack">Return
            </button>
          </div>
          <div v-if="!qrSuccess" class="mb-4 text-color-pink text-center alert-card">
            <Eyes class="my-4" />
            <h1 class="text-bold font-xl">OPS...</h1>
            <p class="text-semibold font-lg">
              {{ qrResponse }}
            </p>
            <button :class="{ 'mx-auto': !isDesktop && !isLargeDesktop }"
              class="mt-3 primary-button text-bold uppercase font-lg" @click="goBack">Close
            </button>
          </div>
        </div>

      </div>

    </div>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import checkViewport from '~/mixins/checkViewport'
import { useWeb3Store } from '~/stores/web3Store'

import axios from 'axios'

import MainContent from '~/components/MainContent.vue'
import Loader from '~/components/Loader.vue'
import FooterApp from '~/components/FooterApp.vue'
import SadIcon from '~/components/icons/SadIcon.vue'

import IconCalendar from '~/components/icons/IconCalendar.vue'
import IconPin from '~/components/icons/IconPin.vue'
import IconTime from '~/components/icons/IconTime.vue'
import IconTicket from '~/components/icons/IconTicket.vue'
import IconOnline from '~/components/icons/IconOnline.vue'
import Spinner from '~/components/icons/Spinner.vue'

export default {
  name: 'home-page',
  mixins: [checkViewport],
  components: {
    MainContent,
    Loader,
    FooterApp,
    SadIcon,
    IconCalendar,
    IconPin,
    IconTime,
    IconTicket,
    IconOnline,
    Spinner,
  },
  data() {
    return {
      apiURL: import.meta.env.VITE_API_URL,
      isLoading: false,
      workingMessage: "Signing to checkpoint, wait..",
      qrResponse: "",
      qrSuccess: false,
      env: useRuntimeConfig(),
      collecting: false,
      checkpoint: null,
      proof: null,
      apiURL: import.meta.env.VITE_API_URL
    }
  },
  computed: {
    ...mapStores(useWeb3Store)
  },
  async mounted() {
    const id = await axios.get(this.apiURL + "/checkpoint/id/" + this.$route.params.proof);
    console.log(id.data);
    const challenge = await axios.get(
      this.apiURL + "/checkpoint/challenge/" + id.data.checkpoint_id
    );
    this.proof = challenge.data.challenge
    const userHasPassport = await this.userPassportExists()
    if (!userHasPassport) {
      const temporaryRoute = `/collect/${this.$route.params.proof}`
      this.saveTemporaryRoute(temporaryRoute)
      this.$router.push('/')
    }
    await this.getCheckpoint()
  },
  methods: {
    goBack() {
      this.$router.push('/')
    },
    saveTemporaryRoute(route) {
      console.log("Saving temporary route", route)
      localStorage.setItem('temporaryRoute', route)
    },
    async userPassportExists() {
      if (!this.web3Store.account) {
        return false
      }
      return await axios.get(`${this.apiURL}/passport/check/${this.web3Store.account}`)
        .then(response => {
          return response?.data?.error
        }).catch(error => {
          console.log(error)
          return false
        })
    },
    async getCheckpoint() {
      const app = this
      this.isLoading = true
      this.workingMessage = "Fetching checkpoint data, wait..."
      try {
        const metadata = await axios.get(`${this.apiURL}/checkpoint/metadata/${this.proof.split('100001')[0]}`)
        console.log("CHECKPOINT METADATA:", metadata.data)
        if (metadata.data.error) {
          app.qrResponse = metadata.data.message
          app.qrSuccess = false
          app.collecting = true
        } else {
          app.qrSuccess = true
          app.checkpoint = metadata.data
        }
        app.isLoading = false
      } catch (e) {
        console.log("Error while fetching checkpoint data", e.message)
      }
    },
    async collectSignature() {
      const app = this
      app.isLoading = false
      app.$forceUpdate()

      if (app.web3Store.isDebug) {
        console.log('🚀 Init collectSignature')
      }
      try {
        let wallet_address = localStorage.getItem('wallet_address')
        if (wallet_address === null || wallet_address === undefined || wallet_address === "" || wallet_address === "null") {
          let url = new URL(window.location.href)
          wallet_address = url.searchParams.get("loggedAs")
          localStorage.setItem('wallet_address', wallet_address)
        }
        if (wallet_address.indexOf("0x") === 0) {
          app.workingMessage = "Collecting signature, wait..."
          app.isLoading = true
          app.collecting = true
          try {
            const collected = await axios.get(`${app.apiURL}/checkpoint/sign/${wallet_address}/${app.proof}`)
            app.isLoading = false
            if (collected.data.message === "Can't store sign receipt.") {
              app.qrResponse = "You already have this certificate"
            } else {
              app.qrResponse = collected.data.message
            }
            app.qrSuccess = !collected.data.error
          } catch (e) {
            console.log("Error while collecting signature", e.message)
            app.isLoading = false
            app.qrResponse = "Error while collecting signature"
            app.qrSuccess = false
            app.collecting = false
          }
        } else {
          app.isLoading = false
          app.qrResponse = "Account not valid!"
          app.qrSuccess = false
          app.collecting = false
        }
      } catch (e) {
        console.log("Error while fetching nfts", e.message)
      } finally {
        app.isLoading = false
      }
    },
  }
}
</script>