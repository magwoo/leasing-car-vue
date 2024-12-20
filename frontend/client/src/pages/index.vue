<script setup lang="ts">
import Calculator from "@/components/calculator/Calculator.vue";
import UHeader from "@/components/header/UHeader.vue";
import UModal from "@/components/UModal.vue";
import Form from "@/components/form/Form.vue";
import { ref } from "vue";

const isFormOpen = ref<boolean>(false);
</script>

<template>
  <UModal v-model="isFormOpen">
    <Form @close="isFormOpen = false" />
  </UModal>
  <main class="flex flex-col px-6 xl:px-12 pt-6 gap-[550px] sm:gap-24">
    <UHeader @open-form="isFormOpen = true" />
    <Calculator @open-form="isFormOpen = true" />
  </main>
</template>
