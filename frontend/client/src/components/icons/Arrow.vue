<script setup lang="ts">
interface Props {
  class?: string;
}

const props = defineProps<Props>();
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    height="24px"
    viewBox="0 -960 960 960"
    width="24px"
    :class="props.class"
    fill="#FFFFFF"
  >
    <path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
  </svg>
</template>
