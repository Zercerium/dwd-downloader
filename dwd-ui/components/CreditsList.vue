<template>
  <Dialog
    v-model:visible="visible"
    modal
    header="Credits"
    :style="{ width: '50rem' }"
    :breakpoints="{ '1199px': '75vw', '575px': '90vw' }"
  >
    <p>
      <a
        href="https://www.flaticon.com/free-icons/cloud-download"
        target="_blank"
        >App Icon: Cloud download icons created by Stockes Design - Flaticon</a
      >
    </p>
  </Dialog>
</template>

<script setup lang="ts">
const visible = defineModel<boolean>();
</script>
