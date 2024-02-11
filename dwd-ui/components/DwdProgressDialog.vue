<template>
  <Dialog
    v-model:visible="visible"
    modal
    :pt="{
      mask: {
        style: 'backdrop-filter: blur(2px)',
      },
    }"
  >
    <template #container="{ closeCallback }">
      <div
        class="flex flex-col gap-5 px-10 py-7"
        style="
          border-radius: 12px;
          background-image: radial-gradient(
            circle at left top,
            rgb(var(--surface-0)),
            rgb(var(--surface-50))
          );
        "
      >
        <div class="flex items-center gap-2">
          <Button
            label="Cancel"
            text
            class="border-white-alpha-30 w-full border p-4 hover:bg-white/10"
            @click="test"
          ></Button>
        </div>
        <ProgressBar :value="progress"></ProgressBar>
        <Button
          type="button"
          label="Close"
          icon="pi pi-times"
          :loading="processing"
          @click="closeCallback"
        />
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import { throttle } from "~/utils/throttle";

const visible = defineModel<boolean>("visible", {
  required: true,
});
const processing = defineModel<boolean>("processing", {
  required: true,
});

const progress = ref(0);

const app_window = Window.getCurrent();
const unlisten = app_window.listen("dwd-progress-update", (event) => {
  const payload: ProgressUpdate = event.payload as ProgressUpdate;
  update_progress(payload);
});
onBeforeUnmount(async () => {
  await unlisten;
});
const update_progress = throttle((progress_update: ProgressUpdate) => {
  progress.value = progress_update.progress;
});

function test() {
  invoke("async_test", { success: true }).catch((e) => {
    console.log("Error", e);
  });
}
</script>
