<template>
  <div class="d-flex align-items-center justify-content-between checkpoint-detail alert-container bg-blurred">
    <div class="detail-modal">
      <div class="detail-container ">
        <div class="detail-content detail-data-container">
          <div class="d-flex justify-content-between align-items-center w-100">
            <p class="text-semibold font-lg new-line-overflow">{{ name }}</p>
            <p class="text-semibold detail-date">{{ date }}</p>
          </div>
          <div class="d-flex justify-content-center align-items-center w-100 my-3">
            <div class="d-flex detail-container detail-image-container h-full">
              <img v-if="image" :src="image" alt="" class="detail-image"/>
            </div>
          </div>
        </div>
        <div class="detail-footer ">
          <div class="d-flex justify-content-between align-items-center ">
            <div class="flex-grow-1 detail-footer-section description-section bordered-right">
              <p class="text-semibold font-md vt-font new-line-overflow">{{ speaker }}</p>
              <p class="font-sm vt-font new-line-overflow">{{ title }} </p>
            </div>
            <div class="d-flex align-items-center detail-footer-section detail-footer-image">
              <NetWorld class="detail-title-icon"/>
            </div>
          </div>
        </div>
      </div>
      <button class="mt-5 primary-button text-bold uppercase font-lg"
              @click="onButtonClick()">
        {{ buttonLabel }}
      </button>
    </div>
  </div>
</template>

<script>
import {mapStores} from 'pinia'
import {useWeb3Store} from '@/stores/web3Store'

import IconWarning from '@/components/icons/IconWarning.vue'
import ChainAddress from "~/components/icons/ChainAddress.vue";
import NetWorld from "~/components/icons/NetWorld.vue";

export default {
  name: 'checkpoint-details',
  props: {
    checkpoint: {
      type: Object,
      required: true,
    },
    buttonLabel: {
      type: String,
      required: true,
    }
  },
  computed: {
    ...mapStores(useWeb3Store)
  },
  components: {
    NetWorld,
    ChainAddress,
    IconWarning
  },
  data: function () {
    return {
      imageBaseURL: import.meta.env.VITE_IMAGE_BASE_URL + '/',
      name: null,
      description: null,
      image: null,
      speaker: null,
      tags: null,
      title: null,
      date: null,
    }
  },
  mounted() {
    this.name = this.checkpoint?.name || 'Unnamed';
    this.description = this.checkpoint?.description || 'No description';
    this.image = this.checkpoint?.image.replace('ipfs://', this.imageBaseURL) || 'default-checkpoint.png';
    const rawDescription = this.checkpoint?.description || null;
    const descriptions = rawDescription ? rawDescription.split(' - ') : [];
    this.speaker = descriptions && descriptions.length > 0 ? descriptions[0] : 'No speaker';
    let rawInfo = descriptions && descriptions.length > 1 ? descriptions[1] : 'No info';
    rawInfo = rawInfo.split('#')
    this.title = rawInfo[0]
    this.tags = rawInfo.shift()
    const timestamp = this.checkpoint?.timestamp_end
    this.date = new Date(timestamp * 1000).getFullYear()
  },
  methods: {
    onButtonClick() {
      this.$emit('onClick');
    }
  }
}
</script>
