export default {
  data() {
    return {
      isMobile: false,
      isTablet: false,
      isDesktop: false,
      isLargeDesktop: false
    }
  },
  async mounted() {
    this.checkViewport()
    window.addEventListener('resize', () => {
      this.checkViewport()
    })
  },
  methods: {
    checkViewport() {
      const app = this
      if (window.innerWidth < 768) {
        app.isMobile = true
        app.isTablet = false
        app.isDesktop = false
        app.isLargeDesktop = false
      } else if (window.innerWidth < 992 && window.innerWidth > 768) {
        app.isMobile = false
        app.isTablet = true
        app.isDesktop = false
        app.isLargeDesktop = false
      } else if (window.innerWidth < 1799 && window.innerWidth > 992) {
        app.isMobile = false
        app.isTablet = false
        app.isDesktop = true
        app.isLargeDesktop = false
      } else if (window.innerWidth > 1799) {
        app.isMobile = false
        app.isTablet = false
        app.isDesktop = true
        app.isLargeDesktop = true
      }
    }
  }
}
