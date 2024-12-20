<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import UButton from "../UButton.vue";
import Arrow from "../icons/Arrow.vue";

const imgs: string[] = [
  "/car1.png",
  "/car2.webp",
  "/car3.jpg",
  "/car2.webp",
  "/car1.png",
  "/car3.jpg",
];
const currentIndex = ref<number>(1);
const nextProgress = ref<number>(0.0);

onMounted(() => {
  setInterval(() => (nextProgress.value += 0.01), 100);
});

watch(nextProgress, (value) => {
  if (value >= 1.0) {
    nextProgress.value = 0.0;
    currentIndex.value++;
  }
});

watch(currentIndex, (value) => {
  if (value < 0) currentIndex.value = imgs.length - 1;
  if (value > imgs.length - 1) currentIndex.value = 0;
  nextProgress.value = 0.0;
});
</script>

<template>
  <section
    class="relative flex flex-col items-center justify-end p-4 lg:p-11 lg:pl-32 w-full lg:w-auto lg:aspect-[16/9] h-full"
  >
    <img
      v-for="[i, img] in imgs.entries()"
      :src="img"
      alt="caroucel image"
      class="absolute lg:inset-0 max-sm:-left-36 -top-4 sm:-top-16 h-2/3 sm:h-2/3 lg:h-full lg:w-full transition duration-500 object-cover"
      :class="{
        'translate-x-0 opacity-100': i == currentIndex,
        '-translate-x-16 opacity-0': i < currentIndex,
        'translate-x-16 opacity-0': i > currentIndex,
      }"
    />
    <div class="relative flex justify-center lg:justify-between w-full">
      <div class="flex items-end">
        <div class="flex gap-4 items-center">
          <button
            v-for="[i] in imgs.entries()"
            @click="currentIndex = i"
            class="rounded-full transition size-2"
            :class="{
              'scale-150 bg-white': i == currentIndex,
              'bg-neutral/35': i != currentIndex,
            }"
          />
        </div>
      </div>
      <div class="hidden lg:flex gap-4 items-center">
        <UButton
          @click="currentIndex--"
          type="round"
          class="size-8 opacity-35 hover:opacity-50"
          :progress="1.0"
          ><Arrow class="rotate-180"
        /></UButton>
        <UButton
          @click="currentIndex++"
          type="round"
          class="size-10"
          :progress="nextProgress"
          ><Arrow
        /></UButton>
      </div>
    </div>
  </section>
</template>
