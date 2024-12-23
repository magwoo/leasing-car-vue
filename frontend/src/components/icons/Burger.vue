<script setup lang="ts">
import { onMounted, ref } from "vue";

const theme = ref<string>("");

function calcTheme() {
  if (window.innerWidth < 640) theme.value = "#FFFFFF";
  else theme.value = "#000000";
}

onMounted(() => {
  window.addEventListener("resize", calcTheme);
  calcTheme();
});
</script>

<template>
  <svg
    width="32"
    height="32"
    viewBox="0 0 32 32"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <g clip-path="url(#clip0_1_1061)">
      <path
        d="M5.33334 8H26.6667"
        :stroke="theme"
        stroke-width="2.75"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <path
        d="M5.33334 16H26.6667"
        :stroke="theme"
        stroke-width="2.75"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <path
        d="M5.33334 24H21.3333"
        :stroke="theme"
        stroke-width="2.75"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </g>
    <defs>
      <clipPath id="clip0_1_1061">
        <rect width="32" height="32" fill="white" />
      </clipPath>
    </defs>
  </svg>
</template>
