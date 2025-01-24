<template>
  <div>
    <p class="text-center">Share on:</p>
    <div class="text-end d-flex justify-content-center mt-2">
      <ShareNetwork
        class="me-2"
        network="twitter"
        :url="siteUrl"
        :title="
          'Get ready for an amazing time at ' +
          event.event_name +
          '! Click the link below to secure your spot and invite your friends.\n\n'
        "
        hashtags="megotickets"
        twitterUser="mego_tickets"
      >
        <i class="fab fah fa-lg fa-twitter me-2"></i>
      </ShareNetwork>

      <ShareNetwork
        class="me-2"
        network="telegram"
        :url="siteUrl"
        :title="
          'Get ready for an amazing time at ' +
          event.event_name +
          '! Click the link below to secure your spot and invite your friends.\n\n'
        "
      >
        <i class="fab fah fa-lg fa-telegram me-2"></i>
      </ShareNetwork>

      <ShareNetwork
        class="me-2"
        network="WhatsApp"
        :url="siteUrl"
        :title="
          'Get ready for an amazing time at ' +
          event.event_name +
          '! Click the link below to secure your spot and invite your friends.\n\n'
        "
      >
        <i class="fa-brands fa-lg fa-whatsapp me-2"></i>
      </ShareNetwork>

      <div @click="copyToClipboard()" style="cursor: pointer">
        <span v-if="!isCopying"><i class="fa fa-copy me-1"></i></span>
        <span v-if="isCopying"><i class="fa-solid fa-clone me-1"></i></span>
      </div>
    </div>
  </div>
</template>

<script>
import checkViewport from '@/mixins/checkViewport'

export default {
  name: 'sharing-buttons',
  props: ['event'],
  mixins: [checkViewport],
  data() {
    return {
      isCopying: false,
      eventDescription: '',
      siteUrl: ''
    }
  },
  mounted() {
    const app = this
    app.formatDescription()
    app.mainRoute()
  },
  methods: {
    mainRoute() {
      const app = this
      app.siteUrl = window.location.href
      console.log('URL IS', app.siteUrl)
    },
    copyToClipboard() {
      const app = this
      if (app.event.tierId) {
        app.isCopying = true
        let copyText = app.siteUrl
        navigator.clipboard.writeText(copyText).then(() => {
          console.log('Copied to Clipboard')
        })
        setTimeout(function () {
          app.isCopying = false
        }, 2000)
      } else {
        console.log("Nothing to copy, make sure that you're connected!")
        app.isCopying = false
      }
    },
    formatDescription() {
      const app = this
      if (app.event.event_description) {
        // app.eventDescription = app.event_description.replace('<p><br></p>', '')
        // console.log('formatted desc', app.eventDescription)
        const parser = new DOMParser()
        const parsedHtml = parser.parseFromString(app.event.event_description, 'text/html')
        app.eventDescription = parsedHtml.body.textContent
      }
    }
  }
}
</script>