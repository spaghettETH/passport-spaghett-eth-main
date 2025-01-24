<template>
  <div>
    <div class="d-flex justify-content-center">
      <div
        :class="{
          'd-flex': !isMobile,
        }"
      >
        <div
          :class="{
            'ms-3': !isMobile,
          }"
        >
          <p style="font-size: 0.8rem">
            <img
              src="@/components/icons/logo.svg"
              height="15"
              style="margin-right: 10px; margin-bottom: 4px"
            />
            <a
              style="font-size: 0.8rem"
              href="https://mego.tickets/"
              target="_blank"
              >Powered by MEGO</a
            >
          </p>
        </div>
        <!-- <div :class="{
          'ms-2': !isMobile,
        }">
          <a style="font-size: 0.8rem" href="https://www.iubenda.com/privacy-policy/84994680" target="_blank">Privacy
            Policy</a>
        </div>
        <div :class="{
          'ms-2': !isMobile,
        }">
          <a style="font-size: 0.8rem" href="https://www.iubenda.com/terms-and-conditions/84994680" target="_blank">Terms
            and Conditions</a>
        </div> -->
      </div>
    </div>
  </div>
</template>

<script>
import checkViewport from "@/mixins/checkViewport";

export default {
  name: "footer-component",
  mixins: [checkViewport],
};
</script>
