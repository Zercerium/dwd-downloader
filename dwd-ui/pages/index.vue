<template>
  <div>
    <Button label="Show Credits" @click="show_credits = true" />
    <CreditsList v-model:visible="show_credits" />
    <Button label="Test open" @click="open_folder" />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const show_credits = ref(false);

function open_folder() {
  const res = invoke("test_open").catch((e) => {
    console.log("Error", e);
  });
  console.log(res);
}
</script>
