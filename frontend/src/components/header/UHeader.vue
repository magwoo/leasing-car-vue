<script setup lang="ts">
import Caroucel from "./Caroucel.vue";
import UButton from "../UButton.vue";
import UH1 from "../UH1.vue";
import UH2 from "../UH2.vue";
import Nav from "../nav/Nav.vue";

const emit = defineEmits(["openForm"]);
</script>

<template>
  <header class="flex flex-col gap-6">
    <Nav @open-form="emit('openForm')" />
    <section
      class="absolute top-0 right-0 left-0 sm:relative flex justify-end bg-black rounded-b-[32px] sm:rounded-[32px] w-full h-[550px] sm:h-[473px] overflow-clip"
    >
      <div
        class="flex flex-col gap-6 absolute lg:top-12 top-72 sm:top-48 left-12 z-10"
      >
        <UH1 class="text-white"
          >Авто в лизинг для<br />
          физических лиц
        </UH1>
        <UH2 class="text-white">Получите машину за 5 дней</UH2>
        <UButton type="primary" @click="emit('openForm')"
          >Оставить заявку</UButton
        >
      </div>
      <Caroucel />
    </section>
  </header>
</template>
