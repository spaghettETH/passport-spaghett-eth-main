<template>

  <div v-if="isLoading">
    <Loader/>
  </div>

  <div class="d-flex flex-column justify-content-center align-items-center mb-5">
    <div v-if="!scanning" class="d-flex flex-column justify-content-center align-items-center">
      <Tongue class="my-3 scan-icon"/>
      <p class="text-color-pink font-lg text-center text-semibold px-4">
        Open the camera and scan the <span class="text-bold">QR Code</span> to get in.
      </p>
    </div>

    <div v-else class="d-flex flex-column justify-content-center align-items-center">
      <div v-if="error">
        {{ error }}
      </div>
      <div v-else class="qr-scanner-container">
        <div class="qr-scanner">
          <qrcode-stream :constraints="selectedConstraints"
                         @detect="onDetect" @error="onError" @camera-on="onCameraReady"/>
        </div>

        <div class="qr-cross-icon" @click="toggleScan()">
          <Cross/>
        </div>
        <QRCodeFrame class="qr-frame"/>
      </div>
    </div>

    <button :class="{ 'mx-auto': !isDesktop && !isLargeDesktop }"
            class="mt-3 primary-button text-bold uppercase font-lg "
            @click="toggleScan()">
      {{ scanning ? 'Cancel' : 'Scan' }}
    </button>

  </div>

</template>

<script>
import checkViewport from '@/mixins/checkViewport'
import Tongue from "~/components/icons/Tongue.vue";
import {QrcodeStream} from 'vue-qrcode-reader'
import Cross from "~/components/icons/Cross.vue";
import QRCodeFrame from "~/components/icons/QRCodeFrame.vue";
import Eyes from "~/components/icons/Eyes.vue";


export default {
  name: 'scan-tab',
  components: {
    Eyes,
    QRCodeFrame,
    Cross,
    Tongue,
    QrcodeStream,
  },
  props: [],
  mixins: [checkViewport],
  data() {
    return {
      isLoading: false,
      scanning: false,
      error: '',
      selectedConstraints: {facingMode: 'environment'}
    }
  },
  mounted() {

  },
  methods: {
    toggleScan() {
      this.scanning = !this.scanning
      if (this.scanning) {
        this.isLoading = true
      }
    },
    onDetect(result) {
      console.log('onDetect', result)
      const value = result[0].rawValue
      if (value.startsWith('https://')) {
        const proof = value.split('/collect/')[1] || null
        if (proof) {
          this.$router.push({path: `/collect/${proof}`})
        }
      }
      this.scanning = false
    },
    onError(err) {
      this.error = `[${err.name}]: `

      if (err.name === 'NotAllowedError') {
        this.error += 'you need to grant camera access permission'
      } else if (err.name === 'NotFoundError') {
        this.error += 'no camera on this device'
      } else if (err.name === 'NotSupportedError') {
        this.error += 'secure context required (HTTPS, localhost)'
      } else if (err.name === 'NotReadableError') {
        this.error += 'is the camera already in use?'
      } else if (err.name === 'OverconstrainedError') {
        this.error += 'installed cameras are not suitable'
      } else if (err.name === 'StreamApiNotSupportedError') {
        this.error += 'Stream API is not supported in this browser'
      } else if (err.name === 'InsecureContextError') {
        this.error +=
            'Camera access is only permitted in secure context. Use HTTPS or localhost rather than HTTP.'
      } else {
        this.error += err.message
      }

      console.error('Camera error: ', this.error)
    },
    async onCameraReady() {
      this.isLoading = false
    }
  }
}
</script>