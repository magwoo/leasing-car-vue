<script setup lang="ts">
interface Props {
  href?: string;
}

const props = defineProps<Props>();
</script>

<template>
  <a
    class="font-semibold text-xl lg:text-base text-accent/75 hover:text-accent"
    :href="props.href ?? '#'"
  >
    <slot></slot>
  </a>
</template>
