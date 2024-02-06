import { ask, message } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

export async function check_for_updates(show_message: boolean = true) {
  const update = await check();

  const store = useSettingsStore();
  store.last_update_check = Date.now();

  console.log(update);
  if (!update) {
    // updateText.value = "No updates available";
    if (show_message) {
      await message("nothing to do", "No updates available");
    }
    return;
  } else {
    const version = update.version;
    const answer = await ask(
      `Install Version ${version} now?`,
      "Update available",
    );
    if (answer) {
      const installed = await update
        .downloadAndInstall()
        .then(() => {
          return true;
        })
        .catch((e) => {
          console.error(e);
          return false;
        });
      if (installed) {
        await relaunch();
      }
    }
  }
  // updateText.value = "Update available";
}

export function day_over(time: number) {
  const now = Date.now();
  const day = 24 * 60 * 60 * 1000;
  return now - time > day;
}