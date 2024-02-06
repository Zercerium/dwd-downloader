<script setup lang="ts">
import { save } from "@tauri-apps/plugin-dialog";
import { toast } from "vue3-toastify";
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";

const props = defineProps<{
  title: string;
  resolutions: any;
  resolution_selected: any;
  formats: any;
  format_selected: any;
  assemble_data_type: () => Product;
}>();

function assemble_request(): UniversalRequest {
  const request: UniversalRequest = {
    product: props.assemble_data_type(),
    start: "",
    end: "",
    station: store.storage.station_id,
    coordinates: store.storage.coordinates,
    // delimiter: null,
    file_path: "",
  };
  return request;
}

const emit = defineEmits<{
  "update:resolution_selected": [resolutionSelected: any];
  "update:format_selected": [formatSelected: any];
}>();

const resolution_selected = ref(props.resolution_selected);
watch(resolution_selected, (newValue) => {
  emit("update:resolution_selected", newValue);
});

const format_selected = ref(props.format_selected);
watch(format_selected, (newValue) => {
  emit("update:format_selected", newValue);
});

type ProgressUpdate = {
  progress: number;
};

const appWindow = Window.getCurrent();
const unlisten = appWindow.listen<ProgressUpdate>(
  "dwd_progress_update",
  (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    progress.value = event.payload.progress;
  },
);
onBeforeUnmount(async () => {
  await unlisten;
});

const store = useDwdRequestFormStore();

const processing = ref(false);
const progress = ref(0);

async function request(f: () => UniversalRequest) {
  processing.value = true;
  const request = f();
  try {
    const start = normalizeDateTime(store.storage.start_date_time);
    const end = normalizeDateTime(store.storage.end_date_time);
    if (start == null || end == null) {
      toast.error("Invalid date");
      return;
    }
    request.start = start;
    request.end = end;
    const filename_suggestion = await invoke<String>(
      "dwd_filename_suggestion",
      {
        request,
      },
    );
    const path = await save({
      filters: [
        {
          name: "CSV",
          extensions: ["csv"],
        },
      ],
      defaultPath: filename_suggestion.toString(),
    });

    if (path == null) {
      return;
    }
    request.file_path = path;
    await invoke<number>("dwd_request", {
      request,
    });
  } finally {
    processing.value = false;
  }
}
</script>

<template>
  <form>
    <div class="mx-4 max-w-2xl pb-4">
      <div class="space-y-12">
        <div class="border-b border-primary-100 pb-12">
          <h2 class="font-bold leading-7">{{ title }}</h2>
          <slot name="description"></slot>

          <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
            <div class="sm:col-span-3">
              <div class="flex flex-col gap-2">
                <label>Start Date</label>
                <InputText
                  v-model="store.storage.start_date_time"
                  placeholder="2022-06-07"
                />
                <small>Format: YYYY-MM-DD</small>
              </div>
            </div>

            <div class="sm:col-span-3">
              <div class="flex flex-col gap-2">
                <label>End Date</label>
                <InputText
                  v-model="store.storage.end_date_time"
                  placeholder="2022-07-07"
                />
                <small>Format: YYYY-MM-DD</small>
              </div>
            </div>

            <slot name="additionalFormData"></slot>

            <div class="sm:col-span-3 sm:col-start-1">
              <div class="flex flex-col gap-2">
                <label>Resolution</label>
                <Dropdown
                  v-model="resolution_selected"
                  :options="resolutions"
                  option-label="label"
                  option-value="idStr"
                  placeholder="Select a resolution"
                  class="w-full md:w-full"
                />
              </div>
            </div>

            <div class="sm:col-span-3">
              <div class="flex flex-col gap-2">
                <label>Format</label>
                <Dropdown
                  v-model="format_selected"
                  :options="formats"
                  option-label="label"
                  option-value="idStr"
                  placeholder="Select a format"
                  class="w-full md:w-full"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="mt-6 flex items-center justify-end gap-x-6">
        <Button
          :disabled="processing"
          label="Load Data"
          @click.prevent="request(assemble_request)"
        />
      </div>
      <ProgressBar
        v-show="processing"
        class="progress"
        :value="progress"
        max="100"
      ></ProgressBar>
    </div>
  </form>
</template>
