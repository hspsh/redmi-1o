use esp_idf_svc::{
    eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi,
};
use heapless::String;

use esp_idf_svc::sntp::EspSntp;


use esp_idf_hal::delay::FreeRtos;

use anyhow::Result;
use log::*;

pub struct WifiManager {
    wifi: EspWifi<'static>,
}

impl WifiManager {
    pub fn new(modem: esp_idf_hal::modem::Modem) -> Result<Self> {
        let nvs = EspDefaultNvsPartition::take()?;
        let sysloop = EspSystemEventLoop::take()?;
        let wifi = EspWifi::new(modem, sysloop, Some(nvs))?;

        Ok(Self { wifi })
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
            FreeRtos::delay_ms(100);
        }

        info!("Connected to WiFi");
        Ok(())
    }

    pub fn sync_time(&self) -> Result<()> {
        let mut retry = 0;
        while retry < 5 {
            if esp_idf_svc::sntp::EspSntp::new_default().is_ok() {
                info!("Time synchronized successfully");
                return Ok(());
            }
            FreeRtos::delay_ms(1000);
            retry += 1;
        }
        Err(anyhow::anyhow!("Failed to sync time"))

        // let sntp = EspSntp::new_default()?;
        // sntp.set_time_sync_enabled(true)?;



    }
}
