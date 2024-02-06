<script setup lang="ts">
const store = useDwdRequestFormStore();

const resolutions: { label: string; idStr: EvaporationResolution }[] = [
  { label: "DailyP", idStr: "EvaporationDailyP" },
  { label: "DailyR", idStr: "EvaporationDailyR" },
  { label: "MonthlyP", idStr: "EvaporationMonthlyP" },
  { label: "MonthlyR", idStr: "EvaporationMonthlyR" },
];

const formats: { label: string; idStr: EvaporationFormat }[] = [
  { label: "Default", idStr: "Default" },
  { label: "SwmmRainfallData", idStr: "SwmmRainfallData" },
];

function assemble_data_type(): Product {
  const request = { Evaporation: store.storage.evaporation };
  return request;
}
</script>

<template>
  <DwdCommonForm
    v-model:format_selected="store.storage.evaporation.format"
    v-model:resolution_selected="store.storage.evaporation.resolution"
    title="Evaporation"
    :assemble_data_type="assemble_data_type"
    :formats="formats"
    :resolutions="resolutions"
  >
    <template #description>
      <p class="mt-1 text-sm leading-6">
        Download von radargestützten Verdunstungsdaten zur Verwendung im QGIS
        Plugin „Generate Swmm inp“ (Standard) oder zum Import in SWMM als
        Verdungstungsdatei (SWMM rainfall data file)<br />
        <a
          href="https://opendata.dwd.de/climate_environment/CDC/grids_germany/daily/evapo_p/BESCHREIBUNG_gridsgermany_daily_evapo_p_de.pdf"
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
