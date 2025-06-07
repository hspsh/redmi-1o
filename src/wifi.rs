use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    sntp::{OperatingMode, SntpConf},
    wifi::EspWifi,
};
use heapless::String;

use esp_idf_svc::sntp::EspSntp;

use esp_idf_hal::delay::FreeRtos;

use anyhow::Result;
use log::*;

pub struct WifiManager {
    wifi: EspWifi<'static>,
    sntp: Option<EspSntp<'static>>,
}

impl WifiManager {
    pub fn new(modem: esp_idf_hal::modem::Modem) -> Result<Self> {
        let nvs = EspDefaultNvsPartition::take()?;
        let sysloop = EspSystemEventLoop::take()?;
        let wifi = EspWifi::new(modem, sysloop, Some(nvs))?;

        Ok(Self { wifi, sntp: None })
    }

    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<()> {
        let mut config = esp_idf_svc::wifi::ClientConfiguration::default();
        config.ssid = String::<32>::try_from(ssid)
            .map_err(|_| anyhow::anyhow!("SSID too long"))?;
        config.password = String::<64>::try_from(password)
            .map_err(|_| anyhow::anyhow!("Password too long"))?;

        self.wifi.set_configuration(
            &esp_idf_svc::wifi::Configuration::Client(config),
        )?;
        self.wifi.start()?;
        self.wifi.connect()?;

        while !self.wifi.is_connected()? {
            FreeRtos::delay_ms(1000);
            log::info!("dupa");
        }

        log::info!("Connected to WiFi");
        Ok(())
    }

    pub fn sync_time(&mut self) -> anyhow::Result<()> {
        if self.sntp.is_none() {
            let sntp_conf = SntpConf {
                servers: ["pool.ntp.org"],
                operating_mode: OperatingMode::Poll,
                sync_mode: esp_idf_svc::sntp::SyncMode::Smooth,
            };
            // let sntp = EspSntp::new(&sntp_conf)?;
            let sntp = EspSntp::new_default()?;
            info!("SNTP initialized");
            self.sntp = Some(sntp);
        }
        Ok(())
    }

    pub fn get_sntp(&self) -> Option<&EspSntp<'static>> {
        self.sntp.as_ref()
    }
}
