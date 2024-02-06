<template>
  <div>
    <Button label="Show Credits" @click="show_credits = true" />
    <CreditsList v-model:visible="show_credits" />
  </div>
</template>

<script setup lang="ts">
const show_credits = ref(false);
</script>
