<script setup lang="ts">
import Cycle from "./icons/Cycle.vue";

type ButtonType = "primary" | "border" | "base" | "round";

interface Props {
  type?: ButtonType;
  class?: string;
  isLoading?: boolean;
  isDisabled?: boolean;
  progress?: number;
}

const buttonStyles = {
  primary: "bg-primary text-white px-4 py-2 hover:bg-accent w-fit ",
  border:
    "bg-white ring-[1px] px-4 py-2 ring-primary text-primary hover:bg-primary hover:text-white w-fit ",
  base: "bg-neutral text-accent px-4 py-2 hover:bg-accent hover:text-white w-fit ",
  round: "relative flex items-center justify-center text-white ",
};

const props = defineProps<Props>();
const emit = defineEmits(["click"]);
</script>

<template>
  <button
    @click="emit('click')"
    class="rounded-full font-semibold transition-colors"
    :class="buttonStyles[props.type ?? 'base'] + props.class"
    :disabled="props.isDisabled"
  >
    <slot v-if="!(isLoading ?? false)"></slot>
    <span v-if="isLoading">loading</span>
    <div v-if="props.type == 'round'" class="absolute inset-0">
      <Cycle
        v-if="props.type == 'round'"
        :progress="props.progress ?? 0.5"
        :size="50"
      />
    </div>
  </button>
</template>
