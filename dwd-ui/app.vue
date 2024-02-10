<template>
  <div
    class="flex h-screen min-h-screen justify-center overflow-x-auto bg-surface-100 text-surface-700 dark:bg-surface-900 dark:text-white/80"
  >
    <div class="sticky top-0 z-20 flex h-full grow">
      <NavBar />
    </div>
    <main class="w-full">
      <div class="px-4 py-4">
        <NuxtLoadingIndicator />
        <NuxtPage />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { day_over } from "./utils/update_check";

const store = useSettingsStore();
if (store.auto_update_check && day_over(store.last_update_check)) {
  check_for_updates(false);
}
</script>

<style></style>
