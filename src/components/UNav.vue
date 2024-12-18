<script setup lang="ts">
import UButton from "./UButton.vue";
import ULink from "./ULink.vue";
</script>

<template>
  <nav class="flex justify-between">
    <div class="flex gap-4 items-center">
      <img src="/logo.svg" alt="logo" />
      <div class="w-[1px] h-[80%] bg-accent/20" />
      <span class="text-accent/50">лизинговая компания</span>
    </div>
    <div class="flex gap-8 items-center">
      <ULink>Лизинг</ULink>
      <ULink>Каталог</ULink>
      <ULink>О нас</ULink>
      <UButton type="border">Оставить заявку</UButton>
    </div>
  </nav>
</template>
