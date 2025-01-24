<template>
  <button class="secondary-button w-8 account-button" @click="showMenu()">
    <div>
      <ChainAddress/>
      {{ compactAddress(web3Store.account) }}
    </div>
  </button>
</template>

<script>
import {mapStores} from 'pinia'
import {useWeb3Store} from '@/stores/web3Store'

import IconWarning from '@/components/icons/IconWarning.vue'
import ChainAddress from "~/components/icons/ChainAddress.vue";
import dataFormatting from "~/mixins/dataFormatting";

export default {
  name: 'alert-message',
  mixins: [dataFormatting],
  computed: {
    ...mapStores(useWeb3Store)
  },
  components: {
    ChainAddress,
    IconWarning
  },
  methods: {
    showMenu() {
      this.web3Store.showConnectModal()
    }
  }
}
</script>
