<template>
  <Menu
    :model="navigation"
    :pt="{
      root: {
        class: `min-w-[12rem] p-1.5 bg-surface-0 dark:bg-surface-700 text-surface-700 dark:text-white/80 ring-1 ring-surface-200 dark:ring-surface-700`,
      },
    }"
  >
    <template #item="{ item, props }">
      <NuxtLink
        v-if="item.route"
        v-slot="{ href, navigate, isActive }"
        :to="item.route"
        custom
      >
        <a
          v-ripple
          :href="href"
          v-bind="props.action"
          :class="[
            isActive
              ? 'bg-surface-100 text-primary-500 dark:bg-surface-300/10 dark:text-primary-400'
              : 'text-surface-700 dark:text-surface-0',
          ]"
          @click="navigate"
        >
          <span
            :class="item.icon"
            class="text-sm"
            style="width: 16px; height: 16px"
          />
          <span class="ml-2 text-sm">{{ item.label }}</span>
        </a>
      </NuxtLink>
    </template>
  </Menu>
</template>

<script setup lang="ts">
const navigation = ref([
  {
    label: "Home",
    icon: "icon-[fa6-solid--house]",
    route: "/",
  },
  {
    label: "Climate",
    icon: "icon-[fa6-solid--temperature-half]",
    route: "/climate",
  },
  {
    label: "Precipitation",
    icon: "icon-[mdi--weather-heavy-rain]",
    route: "/precipitation",
  },
  {
    label: "Radolan",
    icon: "icon-[mdi--world-wide-web]",
    route: "/radolan",
  },
  {
    label: "Evaporation",
    icon: "icon-[mdi--water-evaporation]",
    route: "/evaporation",
  },
  {
    label: "Settings",
    icon: "icon-[fa6-solid--gear]",
    route: "/settings",
  },
]);
// { name: "Home", to: "/", icon: ["fas", "house"] },
//   { name: "Climate", to: "/climate", icon: ["fas", "temperature-half"] },
//   { name: "Precipitation", to: "/precipitation", icon: ["fas", "droplet"] },
//   { name: "Radolan", to: "/radolan", icon: ["fas", "globe"] },
//   {
//     name: "Evaporation",
//     to: "/evaporation",
//     icon: ["fas", "arrow-up-from-ground-water"],
//   },
</script>
