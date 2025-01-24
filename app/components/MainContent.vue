<template>
  <div
    :class="{ 'my-5': isDesktop || isLargeDesktop, 'mb-5': !isDesktop && !isLargeDesktop }"
  >
    <div
      class="row align-items-center"
    >
      <div
        class="col-12 col-md-12 col-lg-6 order-2 order-lg-1"
        :class="{ 'text-center': !isDesktop && !isLargeDesktop }"
      >
        <h4 class="big-title text-color-pink uppercase">
          Sign in to start your collection
        </h4>

        <button
          @click="web3Store.showConnectModal()"
          class="mt-5 primary-button text-bold uppercase font-lg "
          :class="{ 'mx-auto': !isDesktop && !isLargeDesktop }"
        >
          Login
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import {mapStores} from 'pinia'
import {useWeb3Store} from '@/stores/web3Store'

import checkViewport from '@/mixins/checkViewport'

export default {
  name: 'home-content',
  mixins: [checkViewport],
  computed: {
    ...mapStores(useWeb3Store)
  }
}
</script>