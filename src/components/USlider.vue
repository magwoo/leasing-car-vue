<script setup lang="ts">
import { watch } from "vue";

interface Props {
  label: string;
  min: number;
  max: number;
  step?: number;
}

const props = defineProps<Props>();
const model = defineModel<number>();

watch(
  model,
  (v) => {
    model.value = Math.min(props.max, v ?? 0);
    model.value = Math.max(props.min, model.value ?? 0);
  },
  { immediate: true }
);

function caclRangeRatio() {
  if (model.value! > props.max) model.value = props.max;

  return ((model.value ?? props.min) - props.min) / (props.max - props.min);
}
</script>

<template>
  <div class="relative flex flex-col gap-6">
    <label :for="props.label">{{ props.label }}</label>
    <div
      class="flex bg-neutral rounded-2xl items-center p-2 gap-4 font-bold text-3xl text-accent/60"
    >
      <input
        class="grow bg-transparent outline-none appearance-none p-2"
        type="number"
        :min="props.min"
        :max="props.max"
        :value="model"
        @change="(e: any) => model = e.target.value"
        :id="props.label"
      />
      <slot></slot>
      <div class="absolute bottom-0 right-6 left-6">
        <div
          class="h-0.5 bg-primary"
          :style="{
            width: `${caclRangeRatio() * 100}%`,
          }"
        />
      </div>
      <input
        class="absolute bottom-0 right-6 left-6 appearance-none cursor-pointer bg-accent/10 accent-primary h-[1px]"
        type="range"
        v-model="model"
        :step="props.step ?? 1"
        :min="props.min"
        :max="props.max"
      />
    </div>
  </div>
</template>
