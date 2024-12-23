<script setup lang="ts">
import { computed } from "vue";

interface Props {
  progress: number;
}

const props = defineProps<Props>();

const circumference = computed(() => Math.PI * 92);
const dashOffset = computed(() => circumference.value * (1 - props.progress));
</script>

<template>
  <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="block">
    <circle
      class="opacity-30 stroke-neutral"
      cx="50"
      cy="50"
      :r="46"
      fill="none"
      :stroke-width="4"
    />
    <circle
      class="progress-circle stroke-white transition-all"
      cx="50"
      cy="50"
      :r="46"
      fill="none"
      :stroke-width="4"
      :stroke-dasharray="circumference"
      :stroke-dashoffset="dashOffset"
      stroke-linecap="round"
      transform="rotate(-90, 50, 50)"
    />
  </svg>
</template>
