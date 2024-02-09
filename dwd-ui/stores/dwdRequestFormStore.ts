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
        format_config: {
          utc_to_berlin: false,
          offset: 0,
        },
      } as RadolanOptions,
      evaporation: {
        format: "Default",
        resolution: "EvaporationDailyP",
      } as EvaporationOptions,
    },
    localStorage,
    {
      mergeDefaults: false,
      deep: true,
    },
  );

  return {
    storage,
  };
});
