<template>
  <div class="loader-container bg-blurred" style="z-index: 9999;">
    <p class="mb-4 text-color-primary text-bold font-xl uppercase text-shadow">Loading...</p>
    <div class="loader"></div>
  </div>
</template>
<script>
export default {
  name: 'loader-app'
}
</script>

<style lang="scss" scoped>
.loader-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  background: transparent;
}

.loader {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
    box-shadow: 3px -1px 0 var(--primary-color-opaque);
  }

  50% {
    transform: rotate(180deg);
    box-shadow: 3px -1px 0 var(--primary-color-opaque);
  }

  100% {
    transform: rotate(360deg);
    box-shadow: 3px -1px 0 var(--primary-color-opaque);
  }
}
</style>