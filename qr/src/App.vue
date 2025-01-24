<template>
  <div>
    <div id="wrapper">
      <br>
      <div v-if="loaded">
        <img :src="proofQrCode" class="qr-code" /><br>
        <span style="font-size:13px">Last block: {{ proofQrBlock }}</span>
      </div>
      <div v-if="!loaded">
        loading..
      </div>
    </div>
  </div>
</template>

<style scoped></style>

<script>
import QRCode from 'qrcode'
import axios from 'axios';

export default {
  name: "Home",
  data() {
    return {
      axios: axios,
      apiEndpoint: import.meta.env.VITE_API_ENDPOINT,
      appEndpoint: import.meta.env.VITE_APP_ENDPOINT,
      proofQrCode: "",
      proofQrBlock: "",
      checkpoint: "0",
      loaded: false,
      opened: false
    };
  },
  async mounted() {
    this.getProof();
    setInterval(() => {
      this.getProof();
    }, 3000);
  },
  methods: {
    async getProof() {
      const url = new URL(window.location.href);
      const checkpoint = url.searchParams.get("checkpoint");
      if (checkpoint !== undefined && checkpoint !== null) {
        this.checkpoint = checkpoint;
      }
      const id = await this.axios.get(`${this.apiEndpoint}/checkpoint/id/${this.checkpoint}`);
      const response = await this.axios.get(`${this.apiEndpoint}/checkpoint/challenge/${id.data.checkpoint_id}`);
      const proof = response.data.challenge;
      this.proofQrCode = await QRCode.toDataURL(this.appEndpoint + "/collect/" + proof);
      this.proofQrBlock = response.data.block;
      this.loaded = true;
    },
    toggleFullScreen() {
      if (this.opened) {
        document.exitFullscreen();
        this.opened = false;
        return;
      }
      var elem = document.getElementById("wrapper");
      if (elem.requestFullscreen) {
        elem.requestFullscreen();
      } else if (elem.webkitRequestFullscreen) { /* Safari */
        elem.webkitRequestFullscreen();
      } else if (elem.msRequestFullscreen) { /* IE11 */
        elem.msRequestFullscreen();
      }
      this.opened = true
    }
  }
}
</script>
