<script setup lang="ts">
import { ref } from "vue";
import UButton from "../UButton.vue";
import UH1 from "../UH1.vue";
import USlider from "../USlider.vue";
import InfoBlock from "./InfoBlock.vue";

const creditSum = ref<number>(0);
const firstPayment = ref<number>(0);
const months = ref<number>(0);
</script>

<template>
  <UH1
    >Рассчитайте стоимость<br />
    автомобиля в лизинг</UH1
  >
  <div class="grid grid-cols-3 gap-8">
    <USlider
      label="Сумма договора лизинга"
      :min="50000"
      :max="10000000"
      :step="1000"
      v-model="creditSum"
      ><span class="mr-2">₽</span></USlider
    >
    <USlider
      label="Первоначальный взнос"
      :min="0"
      :max="creditSum * 0.8"
      :step="1000"
      v-model="firstPayment"
      ><div class="bg-accent/5 text-xl p-3 rounded-xl">
        {{ Math.round((firstPayment / creditSum) * 100) }}%
      </div></USlider
    >
    <USlider label="Срок лизинга" :min="12" :max="120" v-model="months"
      ><span class="mr-2">мес.</span></USlider
    >
  </div>
  <div class="grid grid-cols-3 gap-8 items-center">
    <InfoBlock label="Сумма договора лизинга"
      >{{ new Intl.NumberFormat("ru-RU").format(creditSum) }} ₽</InfoBlock
    >
    <InfoBlock label="Ежемесячный платеж от"
      >{{
        new Intl.NumberFormat("ru-RU").format(
          Math.ceil((creditSum - firstPayment) / months)
        )
      }}
      ₽</InfoBlock
    >
    <UButton type="primary" class="h-fit w-full py-3 text-2xl"
      >Оставить заявку
    </UButton>
  </div>
</template>
