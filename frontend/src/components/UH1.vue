<script setup lang="ts">
interface Props {
  class?: string;
}

const props = defineProps<Props>();
</script>

<template>
  <h1
    class="text-4xl md:text-[54px] font-bold leading-[1.0]"
    :class="props.class"
  >
    <slot></slot>
  </h1>
</template>
