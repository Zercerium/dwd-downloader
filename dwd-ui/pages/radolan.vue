<script setup lang="ts">
const store = use_dwd_request_form_store();

const resolutions: { label: string; idStr: RadolanResolution }[] = [
  { label: "Min5", idStr: "RadolanMin5" },
  { label: "Min5 Reproc2017", idStr: "RadolanMin5Reproc2017" },
  { label: "Hourly", idStr: "RadolanHourly" },
  { label: "Hourly Reproc 2017", idStr: "RadolanHourlyReproc2017" },
  { label: "Daily", idStr: "RadolanDaily" },
];

const formats: { label: string; idStr: RadolanFormat }[] = [
  { label: "Default", idStr: "Default" },
  { label: "SwmmRainfallData", idStr: "SwmmRainfallData" },
];

function assemble_data_type(): Product {
  const request = { Radolan: store.storage.radolan };
  return request;
}
</script>

<template>
  <DwdCommonForm
    v-model:format_selected="store.storage.radolan.format"
    v-model:resolution_selected="store.storage.radolan.resolution"
    title="Radolan"
    :assemble_data_type="assemble_data_type"
    :formats="formats"
    :resolutions="resolutions"
  >
    <template #description>
      <p class="mt-1 text-sm leading-6">
        Download von radargestützten Niederschlagsdaten zur Verwendung im QGIS
        Plugin „Generate Swmm inp“ (Standard) oder zum Import in SWMM als
        Niederschlagsdatei (SWMM rainfall data file)<br />
        <a
          href="https://www.dwd.de/DE/leistungen/radolan/radarniederschlagsprodukte/radolankurzbeschreibung_pdf.pdf"
          target="_blank"
          >Datensatzbeschreibung
          <span
            class="icon-[heroicons--arrow-top-right-on-square-16-solid] relative top-[.125em] h-4 w-4"
          ></span>
        </a>
      </p>
    </template>

    <template #additionalFormData>
      <div class="sm:col-span-3">
        <div class="flex flex-col gap-2">
          <label>Coordinates</label>
          <Textarea
            v-model="store.storage.coordinates"
            class="!font-mono"
            rows="5"
            cols="30"
            :placeholder="'200,200\n200,201\n201,200\n201,201'"
          />
          <small>x,y pairs; one pair per line</small>
        </div>
      </div>
      <div class="sm:col-span-3">
        <div class="flex flex-col gap-2">
          <label>Format Config</label>
          <div class="flex items-center">
            <Checkbox
              v-model="store.storage.radolan.format_config.utc_to_berlin"
              :binary="true"
            />
            <label
              v-tooltip="
                'Can lead to duplicates if data record includes a time change (the time is set back one hour in the fall)'
              "
              class="ml-2 text-sm"
              >Convert time from UTC to berlin
              <span
                class="icon-[fa6-solid--triangle-exclamation] relative top-[.125em]"
              />
            </label>
          </div>
          <div class="flex items-center">
            <InputNumber
              v-model="store.storage.radolan.format_config.offset"
              class="w-16 [&_input]:text-center"
              :pt-options="{ mergeSections: true, mergeProps: true }"
              :min="-128"
              :max="127"
            />
            <label class="ml-2 text-sm">Time offset</label>
          </div>
        </div>
      </div>
    </template>
  </DwdCommonForm>
</template>
