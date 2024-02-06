import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";

export const useMiscStore = defineStore("misc_store", () => {
  const tauri_version: Ref<string> = ref("unknown");
  const app_version: Ref<string> = ref("unknown");
  const rust_version: Ref<string> = ref("unknown");
  const build_time: Ref<string> = ref("unknown");
  const build_os: Ref<string> = ref("unknown");

  async function init() {
    console.log("Store: misc_store: init");
    tauri_version.value = await getTauriVersion();
    app_version.value = await getVersion();

    const build_infos = await invoke<BuildInfos>("build_infos");
    rust_version.value = build_infos.rust_version;
    build_time.value = build_infos.build_time;
    build_os.value = build_infos.build_os;
  }

  init();

  return {
    tauri_version,
    app_version,
    rust_version,
    build_time,
    build_os,
  };
});
