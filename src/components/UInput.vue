<script setup lang="ts">
interface Props {
  placeholder: string;
  type?: string;
  error?: string;
  isDone?: boolean;
}

const props = defineProps<Props>();
const model = defineModel();
</script>

<template>
  <div class="bg-neutral hover:brightness-95 focus:bg-white text-accent">
    <label :for="props.placeholder">{{ props.placeholder }}</label>
    <input v-model="model" :id="props.placeholder" :type="props.type" />
    <span v-if="props.error">{{ props.error }}</span>
    <span v-if="props.isDone">done</span>
  </div>
</template>
