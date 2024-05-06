<script setup lang="ts">
const store = use_dwd_request_form_store();

const resolutions: { label: string; idStr: ClimateResolution }[] = [
  { label: "Daily", idStr: "ClimateDaily" },
  { label: "Monthly", idStr: "ClimateMonthly" },
  { label: "Annual", idStr: "ClimateAnnual" },
];

const formats: { label: string; idStr: ClimateFormat }[] = [
  { label: "Default", idStr: "Standard" },
];

function assemble_data_type(): Product {
  const request = { Climate: store.storage.climate };
  return request;
}
</script>

<template>
  <DwdCommonForm
    v-model:format_selected="store.storage.climate.format"
    v-model:resolution_selected="store.storage.climate.resolution"
    title="Climate"
    :assemble_data_type="assemble_data_type"
    :formats="formats"
    :resolutions="resolutions"
  >
    <template #description>
      <p class="mt-1 text-sm leading-6">
        Download von stationsbezogenen Klimadaten. <br />
        <a
          href="https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/recent/KL_Tageswerte_Beschreibung_Stationen.txt"
          target="_blank"
          >Liste der Stationen
          <span
            class="icon-[heroicons--arrow-top-right-on-square-16-solid] relative top-[.125em] h-4 w-4"
          ></span>
        </a>
        <br />
        <a
          href="https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/recent/BESCHREIBUNG_obsgermany_climate_daily_kl_recent_de.pdf"
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
          <label>Station ID</label>
          <InputText
            v-model="store.storage.station_id"
            description="Station ID (5 digits)"
            placeholder="04271"
          />
        </div>
      </div>
    </template>
  </DwdCommonForm>
</template>
