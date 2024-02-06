<script setup lang="ts">
import Textarea from "primevue/textarea";

const store = useDwdRequestFormStore();

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
            class="font-mono"
            rows="5"
            cols="30"
            :placeholder="'200,200\n200,201\n201,200\n201,201'"
          />
          <small>x,y pairs; one pair per line</small>
        </div>
      </div>
    </template>
  </DwdCommonForm>
</template>
