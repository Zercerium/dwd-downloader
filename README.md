:warning: Actively in Development and currently unstable :warning:

:construction: (testing and fixing in progress) :construction:

# DWD Downloader

A Downloader for some data from the DWD (Deutscher Wetterdienst)

:information_source: This project is **not** affiliated with the DWD

:information_source: This application is developed during employment at the [Universität Rostock](https://www.uni-rostock.de)

# Usage

![Image from the App](./images/app_foto.png)
- Please don't stress the DWD Server, if the app crashes while the download don't try it again until a fix is deployed
- you can set the ENV `DWD_URL` to use for example a local hosted server which contains DWD data
    - default url: https://opendata.dwd.de/

# Supported Products
- [climate / kl](./infos/climate.md)
    - [daily](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/)
    - [monthly](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/monthly/)
    - [annual](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/annual/)
- [precipitation](./infos/precipitation.md)
    - [Min1]()
    - [Min5]()
    - [Min10]()
    - [Hourly]()
- [radolan](./infos/radolan.md) (WIP)
    - [Min5]()
    - [Min5 Reproc2017]()
    - [Hourly]()
    - [Hourly Reproc2017]()
    - [Daily]()
- [evaporation](./infos/evaporation.md) (WIP)
    - [DailyP]()
    - [DailyR]()
    - [MonthlyP]()
    - [MonthlyR]()

# Attributions

App Icon: [Cloud download icons created by Stockes Design - Flaticon](https://www.flaticon.com/free-icons/cloud-download)

# Development

WIP