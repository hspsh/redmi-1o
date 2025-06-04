use esp_idf_hal::prelude::*;
use esp_idf_svc::{
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
    eventloop::EspSystemEventLoop,
};
use heapless::String;

use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};

use anyhow::Result;
use log::*;

pub struct WifiManager {
    wifi: EspWifi<'static>,
}

impl WifiManager {
    pub fn new(modem: esp_idf_hal::modem::Modem) -> Result<Self> {
        let nvs = EspDefaultNvsPartition::take()?;
        let sysloop = EspSystemEventLoop::take()?;
        let wifi = EspWifi::new(modem, sysloop, None)?;

        Ok(Self { wifi })
    }

    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<()> {
        let mut config = esp_idf_svc::wifi::ClientConfiguration::default();
        config.ssid = String::<32>::try_from(ssid).map_err(|_| anyhow::anyhow!("SSID too long"))?;
        config.password = String::<64>::try_from(password).map_err(|_| anyhow::anyhow!("Password too long"))?;

        self.wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(config))?;
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
            if let Ok(_) = esp_idf_svc::sntp::EspSntp::new_default() {
                info!("Time synchronized successfully");
                return Ok(());
            }
            FreeRtos::delay_ms(1000);
            retry += 1;
        }
        Err(anyhow::anyhow!("Failed to sync time"))
    }
} 