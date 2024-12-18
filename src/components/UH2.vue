<script setup lang="ts">
interface Props {
  class?: string;
}

const props = defineProps<Props>();
</script>

<template>
  <h1 class="text-3xl font-bold" :class="props.class">
    <slot></slot>
  </h1>
</template>
