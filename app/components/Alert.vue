<template>
  <div class="alert-container bg-blurred">
    <div class="px-5 fade-in" style="width: 100%">
      <div class="row justify-content-center">
        <div class="col-12 col-md-4 col-lg-4">
          <div
            class="alert-content d-flex flex-column justify-content-center align-items-center py-4"
          >
            <div class="pb-4 text-center">
              <IconWarning />
              <h3 class="mt-3 color-low-contrast">Attention</h3>
              <!-- <i @click="web3Store.showAlert()" class="fa-solid fa-circle-xmark"></i> -->
            </div>
            <div class="divider"></div>
            <div class="text-center">
              <p class="color-low-contrast my-5">{{ web3Store.alertMessage }}</p>
              <button @click="web3Store.showAlert()" class="mx-auto">
                <span class="text"> Ok</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useWeb3Store } from '@/stores/web3Store'

import IconWarning from '@/components/icons/IconWarning.vue'

export default {
  name: 'alert-message',
  computed: {
    ...mapStores(useWeb3Store)
  },
  components: {
    IconWarning
  }
}
</script>
