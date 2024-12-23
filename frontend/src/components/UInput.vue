<script setup lang="ts">
interface Props {
  placeholder: string;
  type?: string;
  error?: string;
  isDone?: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits(["input"]);
const model = defineModel();
</script>

<template>
  <div
    class="relative bg-neutral hover:brightness-95 text-accent min-w-32 h-12 rounded-2xl"
  >
    <input
      v-model="model"
      @input="(e) => emit('input', e)"
      autocomplete="off"
      :id="props.placeholder"
      :type="props.type"
      placeholder=" "
      class="absolute bottom-1 font-semibold w-full placeholder-shown:inset-0 px-5 peer bg-transparent outline-none transition-all"
    />
    <label
      :for="props.placeholder"
      class="absolute pointer-events-none font-medium top-1 left-5 text-xs peer-placeholder-shown:inset-y-3 peer-placeholder-shown:text-base transition-all"
      >{{ props.placeholder }}</label
    >
    <span v-if="props.error">{{ props.error }}</span>
    <span v-if="props.isDone">done</span>
  </div>
</template>
