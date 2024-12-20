<script setup lang="ts">
import UButton from "../UButton.vue";
import ULink from "../ULink.vue";

const emit = defineEmits(["openForm"]);
</script>

<template>
  <section
    class="fixed right-0 top-0 bottom-0 bg-white rounded-tl-2xl min-w-[260px]"
  >
    <div class="flex flex-col h-svh p-4 sm:p-6">
      <div class="flex flex-col gap-6">
        <button class="self-end text-xl">✕</button>
        <ULink>Лизинг</ULink>
        <ULink>Каталог</ULink>
        <ULink>О нас</ULink>
      </div>
      <div @click.stop class="grow" />
      <UButton @click="emit('openForm')" type="primary" class="w-full"
        >Оставить заявку</UButton
      >
    </div>
  </section>
</template>
