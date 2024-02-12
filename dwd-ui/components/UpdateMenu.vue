<script setup lang="ts">
const update_text = ref("");

const store = use_settings_store();
const date_string = computed(() => {
  if (store.last_update_check) {
    return new Date(store.last_update_check).toLocaleString();
  }
  return "";
});
</script>

<template>
  <div class="grid grid-cols-1 gap-x-6 gap-y-8">
    <div class="flex items-center">
      <InputSwitch v-model="store.auto_update_check" />
      <label class="pl-2">Check for update on startup</label>
    </div>
    <div class="">
      <Button label="Check for updates" @click="check_for_updates()" />
      {{ update_text }}
      Last update check on {{ date_string }}
    </div>
  </div>
</template>
