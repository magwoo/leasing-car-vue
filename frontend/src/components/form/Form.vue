<script setup lang="ts">
import { ref } from "vue";
import UButton from "../UButton.vue";
import UH1 from "../UH1.vue";
import UInput from "../UInput.vue";

const emit = defineEmits(["close"]);

const phone = ref<string>("");
let prevDigits = "";

function applyPhoneMask(e: InputEvent) {
  let digits = phone.value.replace(/[^0-9]/g, "");

  if (!digits.startsWith("7")) digits = digits + digits;

  if (e.data === null && prevDigits.length <= 1) {
    phone.value = "";
    return;
  } else if (e.data === null && prevDigits == digits)
    digits = digits.slice(0, digits.length - 1);

  if (digits.length > 11) digits = digits.slice(0, 11);

  const parts = [
    "+7",
    digits.slice(1, 4),
    digits.slice(4, 7),
    digits.slice(7, 9),
    digits.slice(9, 11),
  ];

  prevDigits = digits;
  phone.value = parts
    .map((part, i) => (part.length == 3 && i == 1 ? "(" + part + ")" : part))
    .map((part, i) => (part.length == 2 && i > 2 ? "-" + part : " " + part))
    .join("")
    .trim();
}
</script>

<template>
  <section
    @click.stop
    class="fixed bottom-0 right-0 left-0 flex justify-center bg-white rounded-t-[32px]"
  >
    <UButton @click="emit('close')" class="absolute top-8 right-8 aspect-square"
      >✕</UButton
    >
    <div
      class="flex flex-col gap-10 w-full max-w-[740px] px-8 pb-[10vh] sm:pb-64 pt-20 md:pt-32"
    >
      <div class="flex flex-col gap-4">
        <UH1>Онлайн-заявка</UH1>
        <span
          >Заполните форму, и мы вскоре свяжемся с вами, чтобы<br />
          ответить на все вопросы</span
        >
      </div>
      <div class="grid md:grid-cols-2 gap-4 md:gap-6">
        <UInput
          v-model="phone"
          type="tel"
          @input="applyPhoneMask"
          placeholder="Телефон"
          class="grow"
        />
        <UInput placeholder="Имя" class="grow" />
        <div
          class="flex flex-col-reverse gap-4 md:col-span-2 items-center md:flex-row justify-between md:p-6 md:ring-1 ring-accent/10 rounded-2xl"
        >
          <span class="text-sm text-accent/50 leading-relaxed"
            >Нажимая на кнопку «Оставить заявку», я даю<br
              class="hidden text-center sm:flex"
            />
            согласие
            <a
              href="#"
              class="text-accent text-center hover:text-primary transition-colors"
              >на обработку персональных данных</a
            ></span
          >
          <UButton class="md:basis-1/3 w-full max-md:h-12" type="primary"
            >Оставить заявку</UButton
          >
        </div>
      </div>
      <div class="flex justify-center gap-4">
        <a
          href="#"
          class="size-14 ring-2 ring-accent/10 p-4 rounded-full hover:ring-accent/20 transition-shadow"
          ><img src="/whatsapp.svg" alt="w" class="h-full w-full"
        /></a>
        <a
          href="#"
          class="size-14 ring-2 ring-accent/10 p-4 rounded-full hover:ring-accent/20 transition-shadow"
          ><img
            src="/telegram.svg"
            alt="t"
            class="h-full w-full -translate-x-[1.5px]"
        /></a>
      </div>
    </div>
  </section>
</template>
