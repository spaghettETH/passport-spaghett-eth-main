<template>
  <div class="loader"></div>
</template>
<script>
export default {
  name: 'loader-app'
}
</script>

<style lang="scss" scoped>
.loader {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
    box-shadow: 0 -1px 0 rgba(255, 255, 255, 1);
  }

  50% {
    transform: rotate(180deg);
    box-shadow: 0 -1px 0 rgba(255, 255, 255, 1);
  }

  100% {
    transform: rotate(360deg);
    box-shadow: 0 -1px 0 rgba(255, 255, 255, 1);
  }
}
</style>