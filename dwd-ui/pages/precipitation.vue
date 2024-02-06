<script setup lang="ts">
const store = useDwdRequestFormStore();

const resolutions: { label: string; idStr: PrecipitationResolution }[] = [
  { label: "Min1", idStr: "PrecipitationMin1" },
  { label: "Min5", idStr: "PrecipitationMin5" },
  { label: "Min10", idStr: "PrecipitationMin10" },
  { label: "Hourly", idStr: "PrecipitationHourly" },
];

const formats: { label: string; idStr: PrecipitationFormat }[] = [
  { label: "Date Together", idStr: "DateTogether" },
  { label: "Date Separated", idStr: "DateSeparated" },
];

function assemble_data_type(): Product {
  const request = { Precipitation: store.storage.precipitation };
  return request;
}
</script>

<template>
  <DwdCommonForm
    v-model:format_selected="store.storage.precipitation.format"
    v-model:resolution_selected="store.storage.precipitation.resolution"
    title="Precipitation"
    :assemble_data_type="assemble_data_type"
    :formats="formats"
    :resolutions="resolutions"
  >
    <template #description>
      <p class="mt-1 text-sm leading-6">
        Download von stationsbezogenen Niederschlagsdaten (mm). <br />
        <a
          href="https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/5_minutes/precipitation/historical/5min_rr_Beschreibung_Stationen.txt"
          target="_blank"
          >Liste der Stationen
          <span
            class="icon-[heroicons--arrow-top-right-on-square-16-solid] relative top-[.125em] h-4 w-4"
          ></span>
        </a>
        <br />
        <a
          href="https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/more_precip/recent/BESCHREIBUNG_obsgermany_climate_daily_more_precip_recent_de.pdf"
          target="_blank"
          >Datensatzbeschreibung
          <span
            class="icon-[heroicons--arrow-top-right-on-square-16-solid] relative top-[.125em] h-4 w-4"
          ></span>
        </a>
        <br />
        Hinweis bei Min1 Daten: An Tagen ohne Regen is die Reihe teilweise
        unvollståndig.
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
