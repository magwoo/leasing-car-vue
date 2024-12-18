<script setup lang="ts">
import { ref } from "vue";

const counter = ref<number>(0);
</script>

<template>
  <button
    @click="counter++"
    class="bg-blue-500 text-white px-3 py-1 rounded-md"
  >
    click me! {{ counter }}
  </button>
</template>
