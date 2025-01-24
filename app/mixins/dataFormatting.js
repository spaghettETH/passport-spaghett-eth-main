export default {
    data() {
    },
    async mounted() {
    },
    methods: {
        checkName(name) {
            if (name && name.length > 0) {
                return name
            } else {
                return 'Anonymous'
            }
        },
        isValidImage(image) {
            return image && image.length > 0 && image.startsWith('https://')
        },
        checkProfileImage(image) {
            if (!this.isValidImage(image)) {
                return 'default-profile.png'
            }
            return image
        },
        checkCheckpointImage(image) {
            if (!this.isValidImage(image)) {
                return 'default-checkpoint.png'
            }
            return image
        },
        compactAddress(address) {
            return `${address.slice(0, 4)}...${address.slice(-4)}`
        }
    }
}
