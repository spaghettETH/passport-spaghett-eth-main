<template>
  <div class="w-100">
    <div v-if="isLoading">
      <Loader />
    </div>

    <div v-else>
      <div class="d-flex flex-column justify-content-center align-items-center w-100 ">
        <div class="leaderboard text-semibold ">
          <div v-for="(leaderboard, index) in leaderboards" :key="index"
            :class="{ 'profile-leaderboard-item': leaderboard.address === web3Store.account }"
            class="leaderboard-item w-100 my-2">
            <div class="leaderboard-item-rank">{{ index + 1 }}</div>
            <img :src="leaderboard.image" alt="avatar" class="avatar">
            <div class="leaderboard-item-name">{{ leaderboard.name }}</div>
            <div class="leaderboard-item-score">{{ leaderboard.score }} pts</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { useWeb3Store } from '@/stores/web3Store'
import checkViewport from '@/mixins/checkViewport'
import axios from "axios";
import { mapStores } from "pinia";
import dataFormatting from "~/mixins/dataFormatting";

export default {
  name: 'leaderboard-tab',
  props: [],
  mixins: [checkViewport, dataFormatting],
  data() {
    return {
      apiURL: import.meta.env.VITE_API_URL,
      isLoading: true,
      showSpinner: true,
      leaderboards: [],
    }
  },
  computed: {
    ...mapStores(useWeb3Store)
  },
  async mounted() {
    await this.getLeaderboard()
  },
  methods: {
    async getLeaderboard() {
      await axios
        .get(`${this.apiURL}/leaderboard`)
        .then(async (response) => {
          const data = response.data.leaderboard

          for (const key of Object.keys(data)) {
            const address = key
            const score = data[key]
            if (!this.leaderboards.includes(address)) {
              const profile = response.data.users.find(user => user.address.toLowerCase() === address.toLowerCase())
              const member = {
                address: address,
                score: score || 0,
                name: this.checkName(profile?.name),
                description: profile?.description,
                image: this.checkProfileImage(profile?.image),
              }
              this.leaderboards.push(member)
            }
          }
        })
        .catch(error => {
          console.log(error)
        })
        .finally(() => {
          this.leaderboards.sort((a, b) => {
            if (a.score !== b.score) {
              return b.score - a.score
            } else {
              return a.address.localeCompare(b.address)
            }
          })
          this.isLoading = false
        })
    }
  }
}
</script>