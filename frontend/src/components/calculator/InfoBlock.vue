<script setup lang="ts">
import UH1 from "../UH1.vue";

interface Props {
  label: string;
}

const props = defineProps<Props>();
</script>

<template>
  <div class="flex flex-col gap-2">
    <span class="text-accent/50">{{ props.label }}</span>
    <UH1 class="text-accent/60"><slot></slot></UH1>
  </div>
</template>
