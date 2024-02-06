<template>
  <div class="flex flex-col gap-y-2">
    <h1>Settings</h1>

    <UpdateMenu />

    <div class="card flex justify-center">
      <Dropdown
        v-model="colorMode.preference"
        :options="colorModes"
        option-label="name"
        option-value="value"
        placeholder="Select Color Mode"
        class="w-full md:w-[14rem]"
      />
    </div>
    <div class="card flex flex-wrap justify-center gap-2">
      <Chip
        v-tooltip.top="
          'App Version (Build at: ' +
          misc_store.build_time +
          ' for ' +
          misc_store.build_os +
          ')'
        "
        :label="misc_store.app_version"
        image="/app-icon.png"
      />
      <Chip
        v-tooltip.top="'Tauri Version'"
        :label="misc_store.tauri_version"
        icon="icon-[logos--tauri]"
      />
      <Chip
        v-tooltip.top="'Rust Version'"
        :label="misc_store.rust_version.split(' ')[1]"
        icon="icon-[fa6-brands--rust]"
        class="dark:hover:text-red-500"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
const misc_store = useMiscStore();

const colorMode = useColorMode();
const colorModes = ref([
  { name: "System", value: "system" },
  { name: "Light", value: "light" },
  { name: "Dark", value: "dark" },
]);

// function delay(time: number) {
//   return new Promise((resolve) => setTimeout(resolve, time));
// }
// await delay(500).then(() => "test");
</script>
