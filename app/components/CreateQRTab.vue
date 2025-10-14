<template>
  <div class="w-100">
    <div v-if="isLoading">
      <Loader />
    </div>

    <div v-else class="d-flex flex-column justify-content-center align-items-center mb-5">
      <!-- Upload Section -->
      <div v-if="!csvData.length" class="d-flex flex-column justify-content-center align-items-center">
        <Spiral class="my-3 scan-icon" />
        <p class="text-color-pink font-lg text-center text-semibold px-4">
          Upload a <span class="text-bold">CSV</span> file to generate checkpoints QR codes. 
          <a href="https://spaghetteth.mypinata.cloud/ipfs/bafkreihsrwojlkrteshycsgo2gc32vaf7malbcmrgx3g3atpzr45ajeyla" target="_blank" class="example-link">Example</a>
        </p>

        <div class="upload-area mt-4" @click="triggerFileInput" @dragover.prevent @drop.prevent="handleDrop">
          <input
            ref="fileInput"
            type="file"
            accept=".csv"
            @change="handleFileUpload"
            style="display: none"
          />
          <p class="text-semibold">Click or drag your CSV file here</p>
          <p class="font-sm mt-2">Format: Speaker, Talk, Tags, Logo, Slug, EventName, StartTime, EndTime</p>
        </div>
      </div>

      <!-- Preview Section -->
      <div v-if="csvData.length && !isGenerating" class="w-100">
        <div class="d-flex flex-column justify-content-center align-items-center">
          <h3 class="text-color-pink font-lg text-semibold mb-3">
            {{ csvData.length }} eventi trovati
          </h3>

          <!-- Preview list -->
          <div class="preview-list w-100">
            <div
              v-for="(event, index) in csvData"
              :key="index"
              class="preview-item d-flex align-items-center justify-content-between"
            >
              <div class="preview-number">{{ (index + 1).toString().padStart(3, '0') }}</div>
              <div class="flex-grow-1 px-3">
                <p class="text-semibold ellipsis">{{ event.Talk }}</p>
                <p class="font-sm ellipsis">{{ event.Speaker }}</p>
              </div>
              <div class="preview-slug font-sm">{{ event.Slug }}</div>
            </div>
          </div>

          <!-- Buttons -->
          <div class="d-flex gap-3 mt-4">
            <button
              class="secondary-button text-bold uppercase font-md"
              @click="resetUpload"
            >
              Annulla
            </button>
            <button
              class="primary-button text-bold uppercase font-md"
              @click="generateQRCodes"
            >
              Genera QR Codes
            </button>
          </div>
        </div>
      </div>

      <!-- Generating Section -->
      <div v-if="isGenerating" class="d-flex flex-column justify-content-center align-items-center">
        <Loader />
        <p class="text-color-pink font-lg text-center text-semibold px-4 mt-3">
          Generazione in corso... {{ generationProgress }}/{{ csvData.length }}
        </p>
      </div>

      <!-- Error Section -->
      <div v-if="error" class="alert alert-error mt-3">
        <p class="text-color-pink font-md text-center">{{ error }}</p>
        <button class="primary-button text-bold uppercase font-sm mt-2" @click="error = ''">
          Chiudi
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import checkViewport from '@/mixins/checkViewport'
import Spiral from '~/components/icons/Spiral.vue'
import Loader from '~/components/Loader.vue'
import Papa from 'papaparse'
import QRCode from 'qrcode'
import JSZip from 'jszip'
import { saveAs } from 'file-saver'

export default {
  name: 'create-qr-tab',
  components: {
    Spiral,
    Loader,
  },
  mixins: [checkViewport],
  data() {
    return {
      isLoading: false,
      isGenerating: false,
      csvData: [],
      error: '',
      generationProgress: 0,
    }
  },
  methods: {
    triggerFileInput() {
      this.$refs.fileInput.click()
    },

    handleDrop(event) {
      const files = event.dataTransfer.files
      if (files.length > 0) {
        this.processFile(files[0])
      }
    },

    handleFileUpload(event) {
      const file = event.target.files[0]
      if (file) {
        this.processFile(file)
      }
    },

    processFile(file) {
      if (!file.name.endsWith('.csv')) {
        this.error = 'Per favore carica un file CSV valido'
        return
      }

      this.isLoading = true
      this.error = ''

      Papa.parse(file, {
        header: true,
        skipEmptyLines: true,
        complete: (results) => {
          if (results.errors.length > 0) {
            this.error = 'Errore nel parsing del CSV: ' + results.errors[0].message
            this.isLoading = false
            return
          }

          // Validate CSV structure
          const requiredFields = ['Speaker', 'Talk', 'Tags', 'Logo', 'Slug', 'EventName', 'StartTime', 'EndTime']
          const hasAllFields = requiredFields.every(field => 
            results.meta.fields.includes(field)
          )

          if (!hasAllFields) {
            this.error = 'Il CSV non ha tutti i campi richiesti: ' + requiredFields.join(', ')
            this.isLoading = false
            return
          }

          this.csvData = results.data
          this.isLoading = false
        },
        error: (error) => {
          this.error = 'Errore nella lettura del file: ' + error.message
          this.isLoading = false
        },
      })
    },

    async generateQRCodes() {
      this.isGenerating = true
      this.generationProgress = 0

      try {
        const zip = new JSZip()
        const baseUrl = 'https://passport.spaghett-eth.com/collect/'

        for (let i = 0; i < this.csvData.length; i++) {
          const event = this.csvData[i]
          const orderedName = (i + 1).toString().padStart(3, '0')
          const filename = `${orderedName}_${event.Slug}.png`

          // Generate QR code as data URL
          const qrDataUrl = await QRCode.toDataURL(baseUrl + event.Slug, {
            width: 512,
            margin: 2,
            color: {
              dark: '#000000',
              light: '#FFFFFF',
            },
          })

          // Convert data URL to blob
          const response = await fetch(qrDataUrl)
          const blob = await response.blob()

          // Add to zip
          zip.file(filename, blob)

          this.generationProgress = i + 1
        }

        // Generate and download ZIP
        const zipBlob = await zip.generateAsync({ type: 'blob' })
        const timestamp = new Date().toISOString().split('T')[0]
        saveAs(zipBlob, `qrcodes_${timestamp}.zip`)

        // Reset
        this.resetUpload()
      } catch (err) {
        console.error('Error generating QR codes:', err)
        this.error = 'Errore nella generazione dei QR codes: ' + err.message
      } finally {
        this.isGenerating = false
        this.generationProgress = 0
      }
    },

    resetUpload() {
      this.csvData = []
      this.error = ''
      this.isGenerating = false
      this.generationProgress = 0
      if (this.$refs.fileInput) {
        this.$refs.fileInput.value = ''
      }
    },
  },
}
</script>

<style scoped>
.upload-area {
  border: 2px dashed var(--secondary-color);
  border-radius: 12px;
  padding: 40px 20px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 300px;
  max-width: 500px;
  width: 100%;
}

.upload-area:hover {
  background: rgba(252, 124, 173, 0.1);
  border-color: var(--primary-color);
}

.preview-list {
  max-height: 400px;
  overflow-y: auto;
  padding: 0 10px;
}

.preview-item {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 8px;
  border: 1px solid var(--text-color-disabled);
}

.preview-number {
  background: var(--secondary-color);
  color: var(--text-color);
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  flex-shrink: 0;
}

.preview-slug {
  color: var(--secondary-color);
  font-family: 'VT323', monospace;
  flex-shrink: 0;
  max-width: 150px;
  font-size: 1rem;
}

.gap-3 {
  gap: 1rem;
}

.alert-error {
  background: rgba(252, 124, 173, 0.1);
  border: 1px solid var(--secondary-color);
  border-radius: 8px;
  padding: 20px;
  text-align: center;
}

.example-link {
  text-decoration: underline;
}

.example-link:hover {
  opacity: 0.8;
}
</style>

