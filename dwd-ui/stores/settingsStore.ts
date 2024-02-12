import { Store } from "@tauri-apps/plugin-store";

export const use_settings_store = defineStore("settings", () => {
  // if true, check on app startup for updates
  // TODO: only check once a day? for updates
  const auto_update_check: Ref<boolean> = ref(false);
  const last_update_check: Ref<number> = ref(0);

  async function init() {
    const store = new Store(".settings.dat");
    await store.get<boolean>("auto_update_check").then((value) => {
      if (value) {
        auto_update_check.value = value;
      }
    });
    await store.get<number>("last_update_check").then((value) => {
      if (value) {
        last_update_check.value = value;
      }
    });
  }

  init();

  watch(auto_update_check, (value) => {
    const store = new Store(".settings.dat");
    store.set("auto_update_check", value);
  });

  watch(last_update_check, (value) => {
    const store = new Store(".settings.dat");
    store.set("last_update_check", value);
  });

  return {
    auto_update_check,
    last_update_check,
  };
});
