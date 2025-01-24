<script setup>
import UserInfo from "~/components/UserInfo.vue";
import Spiral from "~/components/icons/Spiral.vue";
import CollectionIcon from "~/components/icons/CollectionIcon.vue";
import ScanIcon from "~/components/icons/ScanIcon.vue";
import LeaderboardIcon from "~/components/icons/LeaderboardIcon.vue";
import ScanTab from "~/components/ScanTab.vue";
import NetWorld from "~/components/icons/NetWorld.vue";
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
<style>
.nft-image {
  border-radius: 50%;
  margin-right: 10px;
  position: relative;
  width: 50px;
  height: 50px;
}

.nft-image img {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  display: inline-block;
  position: relative;
}

.qr-image {
  border-radius: 50%;
  position: relative;
  width: calc(100vw / 6 - 12px);
  height: 50px;
  margin: 1px !important;
}

.qr-image img {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  display: inline-block;
  position: relative;
}

.big-qr {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: black;
  text-align: center;
  padding: 30vh 0;
}

.letter-qr {
  background: rgba(0, 0, 0, 0.4);
  top: 0;
  left: 0;
  position: absolute;
  height: 50px;
  width: 50px;
  border-radius: 50%;
  text-align: center;
  line-height: 50px;
  font-size: 20px;

}

.big-qr button {
  width: 165px;
  margin-top: 20px;
}

.big-wrapper {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100vh;
  overflow: hidden;
  z-index: 9999999;
}

.big-background {
  backdrop-filter: blur(5px);
  background: rgba(0, 0, 0, 0.5);
  position: absolute;
  z-index: -1;
  top: 0;
  left: 0;
  width: 100%;
  height: 100vh;
}
</style>
<template>
  <div style="padding: 0 15px">
    <div v-if="isLoading">
      <Loader/>
    </div>

    <div v-if="!web3Store.account && !isLoading">
      <MainContent/>
    </div>

    <div v-if="web3Store.account && !isLoading">
      <UserInfo/>
    </div>
    <div v-if="!isLoading && web3Store.account && !hasPassport">
      <div :class="{ 'my-4': isDesktop || isLargeDesktop, 'my-2': !isDesktop && !isLargeDesktop }"
           class="d-flex flex-column justify-content-center align-items-center">
        <Spiral class="my-3"/>
        <h2 class="text-color-pink font-lg text-center text-semibold px-4">
          You don't have any <br/> <span class="text-bold">PassPort</span> yet. Claim it!
        </h2>

        <button class="mt-5 primary-button text-bold uppercase font-lg "
                @click="claimPassport()">
          Claim
        </button>
      </div>
    </div>

    <div v-if="!isLoading && web3Store.account && hasPassport">

      <div class="d-flex justify-content-between align-item-center text-inactive">
        <div :class="{ 'text-color-pink': selectedTab === SCAN }" class="tab-item" @click="selectedTab = SCAN">
          <ScanIcon class="tab-icon"/>
          <p class="text-semibold">Scan</p>
        </div>
        <div :class="{ 'text-color-pink': selectedTab === COLLECTION }" class="tab-item"
             @click="selectedTab = COLLECTION">
          <CollectionIcon class="tab-icon"/>
          <p class="text-semibold">Collection</p>
        </div>
        <div :class="{ 'text-color-pink': selectedTab === LEADERBOARD }" class="tab-item"
             @click="selectedTab = LEADERBOARD">
          <LeaderboardIcon class="tab-icon"/>
          <p class="text-semibold">Leaderboard</p>
        </div>
      </div>

      <div class="d-flex align-items-center justify-content-center mt-2 w-100">
        <div v-if="!isLoading && selectedTab === COLLECTION" class="w-100">

          <!-- // SHOW EVENTS FILTERED BY OWNED TICKETS -->
          <div class="d-flex flex-column justify-content-center align-items-center">
            <div class="mb-2"></div>
            <div v-for="checkpoint in checkpoints"
                 v-if="checkpoints && checkpoints.length > 0" :key="checkpoint.txid"
                 class="d-flex align-items-center justify-content-between checkpoint-item"
                 @click="showCheckpointDetails(checkpoint)">

              <div class="checkpoint-container checkpoint-image-container">
                <img :src="checkCheckpointImage(checkpoint?.image)" alt="" class="checkpoint-image"/>
              </div>
              <div class="checkpoint-container checkpoint-data-container">
                <div class="d-flex justify-content-between align-items-center">
                  <p class="text-semibold font-lg ellipsis">{{ checkpoint.name }}</p>
                  <p class="text-semibold checkpoint-date">{{ checkpoint.date }}</p>

                </div>
                <p class="text-semibold font-md ellipsis speaker-name vt-font">{{ checkpoint.speaker }}</p>
                <p class="font-sm ellipsis">{{ checkpoint.tags }}</p>
                <div class="separator"/>
                <div class="d-flex align-items-center">
                  <NetWorld class="checkpoint-title-icon"/>
                  <p class="font-sm ellipsis vt-font">{{ checkpoint.title }}</p>
                </div>
              </div>
            </div>
            <div v-if="checkpoints.length === 0" class="text-bold text-color-pink font-lg text-center mt-3">
              <Eyes class="my-3"/>
              <p class="text-color-pink font-lg text-center text-semibold px-4">
                You don't have <br/> checkpoints!
              </p>
            </div>
          </div>

          <div v-if="selectedCheckpoint">
            <CheckpointDetails :checkpoint="selectedCheckpoint" button-label="Close"
                               @onClick="hideCheckpointDetails"/>
          </div>

        </div>

        <div v-if="!isLoading && selectedTab === SCAN" class="w-100">
          <ScanTab/>
        </div>

        <div v-if="!isLoading && selectedTab === LEADERBOARD" class="w-100">
          <LeaderboardTab/>
        </div>

      </div>

    </div>

  </div>
</template>

<script>
import {mapStores} from 'pinia'
import checkViewport from '~/mixins/checkViewport'
import {useWeb3Store} from '~/stores/web3Store'

import axios from 'axios'
import MainContent from '~/components/MainContent.vue'
import Loader from '~/components/Loader.vue'
import FooterApp from '~/components/FooterApp.vue'
import IconSad from '~/components/icons/IconSad.vue'

import IconCalendar from '~/components/icons/IconCalendar.vue'
import IconPin from '~/components/icons/IconPin.vue'
import IconTime from '~/components/icons/IconTime.vue'
import IconTicket from '~/components/icons/IconTicket.vue'
import IconOnline from '~/components/icons/IconOnline.vue'
import BellIcon from '~/components/icons/BellIcon.vue'
import dataFormatting from "~/mixins/dataFormatting";

const COLLECTION = 'COLLECTION'
const SCAN = 'SCAN'
const LEADERBOARD = 'LEADERBOARD'

export default {
  name: 'home-page',
  mixins: [checkViewport, dataFormatting],
  components: {
    MainContent,
    Loader,
    FooterApp,
    IconSad,
    IconCalendar,
    IconPin,
    IconTime,
    IconTicket,
    IconOnline,
    BellIcon
  },
  data() {
    return {
      apiURL: import.meta.env.VITE_API_URL,
      imageBaseURL: import.meta.env.VITE_IMAGE_BASE_URL + '/',
      checkpoints: [],
      hasPassport: false,
      isMinting: false,
      isLoading: true,
      showTransactions: false,
      showNFT: false,
      env: useRuntimeConfig(),
      nft: {},
      checking: null,
      selectedTab: COLLECTION,
      selectedCheckpoint: null
    }
  },
  computed: {
    ...mapStores(useWeb3Store),

    upcomingEvents() {
      const now = Date.now()
      return this.eventsFiltered.filter((event) => event.timestamp_end > now)
    },
    pastEvents() {
      const now = Date.now()
      return this.eventsFiltered.filter((event) => event.timestamp_end < now)
    },
    hasPastEvents() {
      return this.pastEvents.length > 0
    },

  },
  async mounted() {
    this.selectedCheckpoint = null
    const app = this
    const wallet_address = localStorage.getItem('wallet_address')
    await app.checkPassport()
    if (wallet_address !== null && wallet_address !== undefined && wallet_address.indexOf("0x") !== -1) {
      await app.fetchCheckpoints()
    } else {
      app.isLoading = false
      // CHECK EVENT CHANGED ACCOUNT
      app.checking = setInterval(async () => {
        const internalEnv = localStorage.getItem('internalEv')
        if (internalEnv !== null) {
          console.log('🚀 Account changed')
          await app.fetchCheckpoints()
        }
      }, 300)
    }
    if (app.hasPassport) {
      app.checkTemporaryRoute()
    }
  },
  methods: {
    hex2a(hexx) {
      var hex = hexx.toString(); //force conversion
      var str = '';
      for (var i = 0; i < hex.length; i += 2)
        str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
      return str;
    },
    checkTemporaryRoute() {
      const temporaryRoute = localStorage.getItem('temporaryRoute')
      if (temporaryRoute !== null) {
        console.log('Redirecting to temporary route')
        localStorage.removeItem('temporaryRoute')
        this.$router.push(temporaryRoute)
      }
    },
    async checkPassport() {
      const app = this
      const wallet_address = localStorage.getItem('wallet_address')
      if (wallet_address !== null && wallet_address !== undefined && wallet_address.indexOf("0x") !== -1) {
        localStorage.removeItem('internalEv')
        clearInterval(app.checking)
        app.isLoading = true
        app.$forceUpdate()

        if (app.web3Store.isDebug) {
          console.log('🚀 Init checkPassport')
        }
        try {
          const check = await axios.get(`${app.apiURL}/passport/check/${wallet_address}`)
          console.log("PASSPORT CHECKED:", check.data)
          app.hasPassport = (!check.data.error && check.data.message !== "Passport can be minted")
              || check.data.reason === "PASSPORT_ALREADY_MINTED"
          app.$forceUpdate()
          app.isLoading = false
        } catch (e) {
          console.log("Error while fetching passport", e.message)
        }
      } else {
        app.isLoading = false
      }
    },

    async claimPassport() {
      const app = this
      const wallet_address = localStorage.getItem('wallet_address')
      if (wallet_address !== null && wallet_address !== undefined && wallet_address.indexOf("0x") !== -1) {
        localStorage.removeItem('internalEv')
        clearInterval(app.checking)
        app.isLoading = true
        app.$forceUpdate()

        if (app.web3Store.isDebug) {
          console.log('🚀 Init claimPassport')
        }
        try {
          app.isMinting = true
          const check = await axios.get(`${app.apiURL}/passport/mint/${wallet_address}`)
          console.log("PASSPORT MINTED:", check.data)
          app.hasPassport = !check.data.error || check.data.reason === "PASSPORT_ALREADY_MINTED"
          if (app.hasPassport) {
            await app.checkPassport()
          } else {
            alert(check.data.message)
          }
        } catch (e) {
          console.log("Error while claiming passport", e.message)
        } finally {
          app.isMinting = false
          app.isLoading = false
          if (app.hasPassport) {
            app.checkTemporaryRoute()
          }
        }
      } else {
        app.isLoading = false
      }
    },
    async fetchCheckpoints() {
      const app = this
      const wallet_address = localStorage.getItem('wallet_address')
      if (wallet_address !== null && wallet_address !== undefined && wallet_address.indexOf("0x") !== -1) {
        localStorage.removeItem('internalEv')
        clearInterval(app.checking)
        app.isLoading = true
        const checkpoints = []
        app.$forceUpdate()

        if (app.web3Store.isDebug) {
          console.log('🚀 Init fetchCheckpoints')
        }
        try {
          const collected = await axios.get(`${app.apiURL}/checkpoint/collected/${wallet_address}`)
          console.log("COLLECTED:", collected.data)
          for (let k in collected.data.signs) {
            const sign = collected.data.signs[k]
            const metadata = sign.metadata
            if (metadata !== undefined) {
              const name = sign.metadata?.name || 'Unnamed'
              const rawDescription = sign.metadata?.description || null;
              const image = sign.metadata?.image.replace('ipfs://', this.imageBaseURL) || 'default-checkpoint.png';
              const descriptions = rawDescription ? rawDescription.split(' - ') : [];
              const speaker = descriptions && descriptions.length > 0 ? descriptions[0] : 'No speaker';
              let rawInfo = descriptions && descriptions.length > 1 ? descriptions[1] : 'No info';
              rawInfo = rawInfo.split('#')
              const title = rawInfo.shift()
              const tags = "#" + rawInfo.join(' #')
              const timestamp = sign.metadata?.timestamp_end
              const date = new Date(timestamp * 1000).getFullYear()
              const checkpointData = {name, image, speaker, tags, metadata, title, date}
              checkpoints.push(checkpointData)
            }
          }
          app.$forceUpdate()
          app.isLoading = false
          app.checkpoints = checkpoints
        } catch (e) {
          console.log("Error while fetching nfts", e.message)
        } finally {
          app.isLoading = false
        }
      } else {
        app.isLoading = false
      }
    },
    showCheckpointDetails(checkpoint) {
      this.selectedCheckpoint = checkpoint.metadata
    },
    hideCheckpointDetails() {
      this.selectedCheckpoint = null
    }
  }
}
</script>