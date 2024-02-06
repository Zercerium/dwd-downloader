# Table of Contents

1. [Climate](#Climate)
2. [Precipitation](#Precipitation)
3. [Radolan](#Radolan)

# Climate

### Available Resolutions

- [daily](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/)
- [monthly](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/monthly/kl/)
- [annual](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/annual/kl/)

### Format

#### Standard

```
STATIONS_ID; MESS_DATUM; QN_3; FX; FM; QN_4; RSK; RSKF; SDK;SHK_TAG; NM; VPM; PM; TMK; UPM; TXK; TNK; TGK; eor
...
```

1:1 Input

### Details

<details>
  <summary>Online Structure</summary>

#### daily

- historical: tageswerte_KL_00001_19370101_19860630_hist.zip (\*single?)
  - produkt_klima_tag_18790101_20221231_05440.txt
  - `STATIONS_ID; MESS_DATUM; QN_3; FX; FM; QN_4; RSK; RSKF; SDK;SHK_TAG; NM; VPM; PM; TMK; UPM; TXK; TNK; TGK; eor`
- recent: tageswerte_KL_00078_akt.zip
  - produkt_klima_tag_20211209_20230611_00078.txt
  - `STATIONS_ID; MESS_DATUM; QN_3; FX; FM; QN_4; RSK; RSKF; SDK;SHK_TAG; NM; VPM; PM; TMK; UPM; TXK; TNK; TGK; eor`

#### monthly

- historical: monatswerte_KL_00003_18510101_20110331_hist.zip (\*single?)
  - produkt_klima_monat_18510101_20110331_00003.txt
  - `STATIONS_ID; MESS_DATUM_BEGINN; MESS_DATUM_ENDE; QN_4; MO_N; MO_TT; MO_TX; MO_TN; MO_FK; MX_TX; MX_FX; MX_TN; MO_SD_S; QN_6; MO_RR; MX_RS; eor`
- recent: monatswerte_KL_00078_akt.zip
  - produkt_klima_monat_20211101_20230531_00044.txt
  - `STATIONS_ID; MESS_DATUM_BEGINN; MESS_DATUM_ENDE; QN_4; MO_N;MO_TT; MO_TX; MO_TN; MO_FK; MX_TX; MX_FX; MX_TN; MO_SD_S; QN_6; MO_RR; MX_RS; eor`

#### annual

- historical: jahreswerte_KL_00001_19310101_19860630_hist.zip (\*single?)
  - produkt_klima_jahr_19310101_19860630_00001.txt
  - `STATIONS_ID; MESS_DATUM_BEGINN; MESS_DATUM_ENDE; QN_4; JA_N; JA_TT; JA_TX; JA_TN; JA_FK; JA_SD_S; JA_MX_FX; JA_MX_TX; JA_MX_TN; QN_6; JA_RR; JA_MX_RS; eor`
- recent: jahreswerte_KL_00044_akt.zip
  - produkt_klima_jahr_20180101_20221231_00044.txt
  - `STATIONS_ID; MESS_DATUM_BEGINN; MESS_DATUM_ENDE; QN_4; JA_N; JA_TT; JA_TX; JA_TN; JA_FK; JA_SD_S; JA_MX_FX; JA_MX_TX; JA_MX_TN; QN_6; JA_RR; JA_MX_RS; eor`

</details>

# Precipitation

### Available Resolutions:

- [1_minute](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/1_minute/precipitation/)
- [5_minutes](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/5_minutes/precipitation/)
- [10_minutes](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/precipitation/)
- [hourly](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/hourly/precipitation/)

### Format

(tabstop separated)

#### Standard

```
STATIONS_ID, MESS_DATUM, RS
MESS_DATUM = YYYMMDDhhmm // 202201010000
```

RS = [RS_01, RS_05, RWS_10, R1] depending on resolution

#### Extra

```
Station, Date, Time, Value
Date = MM/DD/JJJJ
Time = hh:mm:ss
```

### Details

<details>
  <summary>Online Structure</summary>

#### 1_minute

- historical
  - [year]/1minutenwerte_nieder_00020_20220101_20220131_hist.zip (\*monthly)
    - produkt_ein_min_rr_20220101_20220131_00020.txt
    - `STATIONS_ID; MESS_DATUM_BEGINN; MESS_DATUM_ENDE; QN; RS_01; RTH_01; RWH_01; RS_IND_01; eor`
- recent
  - 1minutenwerte_nieder_00020_akt.zip
    - produkt_ein_min_rr_20220101_20230612_00020.txt
      `STATIONS_ID; MESS_DATUM; QN; RS_01; RS_IND_01; eor`
- now
  - 1minutenwerte_nieder_00020_now.zip
    - produkt_ein_now_rr_20230612_20230614_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN; RS_01; RS_IND_01; eor`

#### 5_minutes

- historical
  - [year]/5minutenwerte_nieder_00020_20220101_20220131_hist.zip (\*monthly)
    - produkt_5min_hist_rr_20220101_20220131_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN_5min; RS_IND_05; RS_05; RTH_05; RWH_05; eor`
- recent
  - 5minutenwerte_nieder_00020_akt.zip
    - produkt_5min_rr_20220101_20230611_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN_5min; RS_COUNT_05; RS_IND_05; RS_05; eor`
- now
  - 5minutenwerte_nieder_00020_now.zip
    - produkt_5min_now_rr_20230612_20230614_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN_5min; RS_COUNT_05; RS_IND_05; RS_05; eor`

#### 10_minutes

- historical
  - 10minutenwerte_nieder_00003_19930428_19991231_hist.zip (\*multiple different range)
    - produkt_zehn_min_rr_19930428_19991231_00003.txt
    - `STATIONS_ID; MESS_DATUM; QN; RWS_DAU_10; RWS_10; RWS_IND_10`
- recent
  - 10minutenwerte_nieder_00020_akt.zip
    - produkt_zehn_min_rr_20211210_20230612_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN; RWS_DAU_10; RWS_10; RWS_IND_10; eor`
- now
  - 10minutenwerte_nieder_00020_now.zip
    - produkt_zehn_now_rr_20230613_20230613_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN; RWS_DAU_10; RWS_10; RWS_IND_10; eor`

#### hourly

- historical
  - stundenwerte_RR_00003_19950901_20110401_hist.zip (\*single?)
    - produkt_rr_stunde_20040814_20221231_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN_8; R1; RS_IND; WRTR; eor`
- recent
  - stundenwerte_RR_00020_akt.zip
    - produkt_rr_stunde_20211210_20230612_00020.txt
    - `STATIONS_ID; MESS_DATUM; QN_8; R1; RS_IND; WRTR; eor`

</details>

# Radolan

### Available Resolutions

- [5_minutes](https://opendata.dwd.de/climate_environment/CDC/grids_germany/5_minutes/radolan/)
- [hourly](https://opendata.dwd.de/climate_environment/CDC/grids_germany/hourly/radolan/)
- [daily](https://opendata.dwd.de/climate_environment/CDC/grids_germany/daily/radolan/)

### Format

(comma separated)

#### Standard

```
Name, Date, Time, Value
Name = 0200_0100
Date = YYYY-MM-DD
Time = hh:mm
```

sorted after Name, Date, Time in this order

#### SWMM rainfall data file

```
Name, Jahr, Monat, Tag, Stunde, Minute, Wert
0186_0583, 2019, 10, 17, 5, 5, 0
```

sorted after Name, Date, ...

### Details

<details>
  <summary>Online Structure</summary>

#### 5_minutes (reproc/2017_002/bin)

- [year]/YW2017.002_YYYYMM.tar
  - YW2017.002_YYYYMMDD.tar.gz
    - raa01-yw2017.002_10000-YYMMDDhhmm-dwd--bin

#### hourly

- historical (not used)
- recent (not used)
- reproc/2017_002/bin
  - [year]/RW2017.002_202301.tar.gz
    - raa01-rw2017.002_10000-YYMMDDhh50-dwd---bin (hourly takt)

#### daily

- historical/bin (no asc)
  - [year]/SFYYYYMM.tar.gz
    - raa01-sf_10000-YYMMDDhhmm-dwd---bin (hourly takt)
- recent/bin (no asc)
  - raa01-sf_10000-2301010050-dwd---bin.gz
    - raa01-sf_10000-2301010050-dwd---bin

</details>
