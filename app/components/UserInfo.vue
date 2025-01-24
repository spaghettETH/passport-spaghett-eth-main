<template>

  <div v-if="isLoading">
    <Loader/>
  </div>

  <div :class="{ 'mb-4': !isDesktop && !isLargeDesktop }">
    <div class="row align-items-center">
      <div :class="{ 'text-center': !isDesktop && !isLargeDesktop }" class="col-12 col-md-12 col-lg-6 order-2 order-lg-1">
        <AddressButton />
        <div class="mt-1">
          <img v-if="image" :src="image" alt="avatar" class="avatar" @click="editProfile()">
          <EditPen class="edit-avatar-icon" @click="editProfile()" />
        </div>

        <div>
          <div class="text-bold font-lg">
            {{ username }}
          </div>
          <div class="font-sm">
            {{ compactAddress(web3Store.account) }}
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script>
import {mapStores} from 'pinia'
import {useWeb3Store} from '@/stores/web3Store'
import checkViewport from '@/mixins/checkViewport'
import ChainAddress from "~/components/icons/ChainAddress.vue";
import EditPen from "~/components/icons/EditPen.vue";
import dataFormatting from "~/mixins/dataFormatting";
import axios from "axios";

export default {
  name: 'user-info',
  components: { EditPen, ChainAddress },
  mixins: [checkViewport, dataFormatting],
  computed: {
    ...mapStores(useWeb3Store)
  },
  data: function () {
    return {
      apiURL: import.meta.env.VITE_API_URL,
      isLoading: false,
      username: null,
      image: null,
      on: {
        'profile-updated': this.refreshProfile
      }
    }
  },
  async mounted() {
    await this.refreshProfile()
  },
  methods: {
    async refreshProfile() {
      this.isLoading = true
      let address = this.web3Store.account
      if (!address) {
        const account = localStorage.getItem("wallet_address");
        const email = localStorage.getItem("wallet_email");
        if (account) {
          address = account
        } else if (email) {
          address = email
        }
      }
      const userProfile = await this.getProfile(address)
      this.username = this.checkName(userProfile?.name)
      this.image = this.checkProfileImage(userProfile?.image)
      this.isLoading = false
    },
    editProfile() {
      this.$router.push('/profile')
    },
    async getProfile(address) {
      return axios
          .get(`${this.apiURL}/passport/profile/${address}`)
          .then(response => {
            return response.data
          })
          .catch(error => {
            console.log(error)
          })
    }
  }
}
</script>