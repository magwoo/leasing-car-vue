<script setup lang="ts">
import Caroucel from "./Caroucel.vue";
import UButton from "../UButton.vue";
import UH1 from "../UH1.vue";
import UH2 from "../UH2.vue";
import UNav from "../UNav.vue";
</script>

<template>
  <header class="flex flex-col gap-6">
    <UNav />
    <section
      class="relative flex justify-end bg-black rounded-[32px] w-full h-[473px] overflow-clip"
    >
      <div class="flex flex-col gap-6 absolute top-12 left-12 z-10">
        <UH1 class="text-white"
          >Авто в лизинг для<br />
          физических лиц
        </UH1>
        <UH2 class="text-white">Получите машину за 5 дней</UH2>
        <UButton type="primary">Оставить заявку</UButton>
      </div>
      <Caroucel />
    </section>
  </header>
</template>
