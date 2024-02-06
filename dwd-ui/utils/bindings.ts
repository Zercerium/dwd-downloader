// This file has been generated by Specta. DO NOT EDIT.

export type BuildInfos = { rust_version: string; build_time: string; build_os: string }

export type ClimateFormat = "Standard"

export type ClimateOptions = { resolution: ClimateResolution; format: ClimateFormat }

export type ClimateResolution = "ClimateDaily" | "ClimateMonthly" | "ClimateAnnual"

export type EvaporationFormat = "Default" | "SwmmRainfallData"

export type EvaporationOptions = { resolution: EvaporationResolution; format: EvaporationFormat }

export type EvaporationResolution = "EvaporationDailyP" | "EvaporationDailyR" | "EvaporationMonthlyP" | "EvaporationMonthlyR"

export type PrecipitationFormat = "DateTogether" | "DateSeparated"

export type PrecipitationOptions = { resolution: PrecipitationResolution; format: PrecipitationFormat }

export type PrecipitationResolution = "PrecipitationMin1" | "PrecipitationMin5" | "PrecipitationMin10" | "PrecipitationHourly"

export type Product = { Climate: ClimateOptions } | { Precipitation: PrecipitationOptions } | { Radolan: RadolanOptions } | { Evaporation: EvaporationOptions }

export type RadolanFormat = "Default" | "SwmmRainfallData"

export type RadolanOptions = { resolution: RadolanResolution; format: RadolanFormat }

export type RadolanResolution = "RadolanDaily" | "RadolanHourly" | "RadolanHourlyReproc2017" | "RadolanMin5" | "RadolanMin5Reproc2017"

export type UniversalRequest = { start: string; end: string; station: string; coordinates: string; product: Product; file_path: string }
