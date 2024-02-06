import { useStorage } from "@vueuse/core";

export const useDwdRequestFormStore = defineStore("dwdRequestForm", () => {
  const storage = useStorage(
    "dwdRequestFormStore",
    {
      start_date_time: "",
      end_date_time: "",
      station_id: "",
      coordinates: "",
      path: "",

      climate: {
        format: "Standard",
        resolution: "ClimateDaily",
      } as ClimateOptions,
      precipitation: {
        format: "DateTogether",
        resolution: "PrecipitationMin1",
      } as PrecipitationOptions,
      radolan: {
        format: "Default",
        resolution: "RadolanMin5",
      } as RadolanOptions,
      evaporation: {
        format: "Default",
        resolution: "EvaporationDailyP",
      } as EvaporationOptions,
    },
    localStorage,
    {
      mergeDefaults: true,
    },
  );

  return {
    storage,
  };
});
