<script setup lang="ts">
import Burger from "../icons/Burger.vue";
import Logo from "../icons/Logo.vue";
import UButton from "../UButton.vue";
import ULink from "../ULink.vue";
import UModal from "../UModal.vue";
import Sidebar from "./Sidebar.vue";
import { ref } from "vue";

const emit = defineEmits(["openForm"]);

const isSidebarOpen = ref<boolean>(false);
</script>

<template>
  <UModal v-model="isSidebarOpen" class="lg:hidden">
    <Sidebar />
  </UModal>
  <nav class="flex justify-between z-20">
    <div class="flex gap-4 items-center">
      <Logo />
      <div class="w-[1px] h-[80%] hidden sm:flex bg-accent/20" />
      <span class="text-accent/50 hidden sm:flex">лизинговая компания</span>
    </div>
    <div class="hidden lg:flex gap-8 items-center">
      <ULink>Лизинг</ULink>
      <ULink>Каталог</ULink>
      <ULink>О нас</ULink>
      <UButton type="border" @click="emit('openForm')">Оставить заявку</UButton>
    </div>
    <button class="flex lg:hidden" @click="isSidebarOpen = true">
      <Burger class="flex lg:hidden" />
    </button>
  </nav>
</template>
