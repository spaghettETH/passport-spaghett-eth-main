<template>

  <div v-if="isLoading">
    <Loader/>
  </div>

  <div v-if="web3Store.account" class="profile-section">
    <AddressButton/>
    <div class="avatar-heading"></div>

    <div class="big-avatar-section">
      <img :src="newImage ? newImage : image" alt="avatar" class="big-avatar ">

      <form class="upload-form">
        <label class="text-decoration-underline">
          <input accept="image/*" type="file" @change="handleFileChange"/>
          Change picture
        </label>
      </form>
    </div>

    <div class="d-flex flex-column justify-content-center w-100 px-4 update-info-section">
      <p class="text-semibold font-md">Username</p>

      <input :value="newUsername" class="username-input mb-2 mt-1"
             type="text"
             @input="newUsername = $event.target.value">
      <button :disabled="newUsername === username && !newImage" class="black-button w-100 text-semibold"
              @click="updateProfile()">
        Update
      </button>
    </div>
    <div class="d-flex flex-column justify-content-end w-100 px-4 end-container">
      <button class="black-button w-100 text-semibold" @click="goBack()">
        Go Back
      </button>
    </div>

  </div>
</template>

<script>
import checkViewport from "~/mixins/checkViewport";
import {mapStores} from "pinia";
import {useWeb3Store} from "~/stores/web3Store";
import axios from "axios";
import dataFormatting from "~/mixins/dataFormatting";
import ChainAddress from "~/components/icons/ChainAddress.vue";
import {signMessage} from "@wagmi/core";

const SET_PASSPORT_MESSAGE = 'SET PASSPORT PROFILE'

export default {
  name: 'profile-page',
  mixins: [checkViewport, dataFormatting],
  components: {ChainAddress},
  data() {
    return {
      apiURL: import.meta.env.VITE_API_URL,
      ipfsURL: import.meta.env.VITE_UMI_API_URL,
      megoWalletUrl: import.meta.env.VITE_MEGO_WALLET_URL,
      isLoading: false,
      username: null,
      image: null,
      newUsername: null,
      newImage: null,
      megoWalletProvider: null,
      address: null,
      imageUrl: null,
    }
  },
  computed: {
    ...mapStores(useWeb3Store),
  },
  async mounted() {
    this.megoWalletProvider = localStorage.getItem('wallet_provider')
    if (this.megoWalletProvider === null) {
      this.megoWalletProvider = 'email'
    }
    this.address = this.web3Store.account
    if (!this.address) {
      const account = localStorage.getItem("wallet_address");
      const email = localStorage.getItem("wallet_email");
      if (account) {
        this.address = account
      } else if (email) {
        this.address = email
      }
    }

    const urlParams = new URLSearchParams(window.location.search);
    const signature = urlParams.get('signature');
    if (signature) {
      this.newUsername = localStorage.getItem('newUsername')
      this.image = localStorage.getItem('imageUrl')
      await this.updateProfile(signature)
      window.history.replaceState({}, document.title, window.location.pathname);
      localStorage.removeItem('newUsername')
      localStorage.removeItem('imageUrl')
    }

    await this.refreshProfile()
  },
  methods: {
    async refreshProfile() {
      this.isLoading = true
      const userProfile = await this.getProfile(this.address)
      this.setProfileValues(userProfile)
      this.isLoading = false
    },
    setProfileValues(userProfile) {
      this.username = this.checkName(userProfile?.name)
      this.image = this.checkProfileImage(userProfile?.image)
      this.newUsername = this.username
      this.newImage = null
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
    },
    handleFileChange(event) {
      this.newImageFile = event.target.files[0]
      if (this.newImageFile) {
        this.newImage = URL.createObjectURL(this.newImageFile)
      }
    },
    async signMessageWithWagmi() {
      return signMessage({message: SET_PASSPORT_MESSAGE})
    },
    signMessageWithMego(provider) {
      let origin = window.location.href.replace("https://", "").replace("http://", "")
      localStorage.setItem('newUsername', this.newUsername)
      localStorage.setItem('imageUrl', this.imageUrl)
      window.location = `${this.megoWalletUrl}/auth/${provider}?message=${SET_PASSPORT_MESSAGE}&origin=${origin}`
    },
    signMessageWithGoogle() {
      this.signMessageWithMego('google')
    },
    signMessageWithApple() {
      this.signMessageWithMego('apple')
    },
    async getSignature() {
      const walletType = this.web3Store.walletType
      console.log("Wallet type", walletType)
      switch (walletType) {
        case 'mego':
          console.log(this.megoWalletProvider)
          if (this.megoWalletProvider === 'google') {
            console.log("Signing with Google")
            return this.signMessageWithGoogle();
          }
          if (this.megoWalletProvider === 'apple') {
            console.log("Signing with Apple")
            return this.signMessageWithApple();
          }
          console.error("Unknown provider")
          break;
        default:
          console.log("Signing with Wagmi")
          return await this.signMessageWithWagmi()
      }
    },
    async getNewImageUrl() {
      let formData = new FormData();
      formData.append("file", this.newImageFile);
      formData.append("name", this.newImageFile.name);
      return await axios.post(`${this.ipfsURL}/ipfs/upload`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data'
        }
      }).then(response => {
        return response.data['ipfs_link']
      }).catch(error => {
        console.log(error)
        return null
      })
    },
    async updateProfile(signature) {
      this.isLoading = true
      this.imageUrl = this.image
      if (this.newImage) {
        this.imageUrl = await this.getNewImageUrl()
        if (!this.imageUrl) {
          return
        }
      }

      let newProfileData = {
        address: this.address,
        name: this.newUsername,
        image: this.imageUrl,
        description: "No description"
      }
      if (!signature) {
        signature = await this.getSignature()
        if (!signature) {
          this.isLoading = false
          return
        }
      }

      newProfileData.signature = signature

      await axios
          .post(`${this.apiURL}/profile`, newProfileData)
          .then(async (response) => {
            console.log(response)
            await this.refreshProfile()
          })
          .catch(error => {
            console.log(error)
          }).finally(() => {
            this.isLoading = false
            this.$emit('profile-changed')
          })
    },
    goBack() {
      this.$router.push('/')
    }
  }
}
</script>