use std::marker::PhantomData;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;
use esp_idf_hal::gpio::{InputPin, OutputPin};
use esp_idf_hal::{
    gpio::Gpio2,
    ledc::*,
    peripheral::Peripheral,
    prelude::*,
};
use anyhow::Result;

pub struct BuzzerTone {
    pub freq_hz: u32,
    pub duration_ms: u32,
}

pub struct Buzzer<'d> {
    sender: Sender<BuzzerTone>,
    _p: PhantomData<&'d mut ()>,
}

impl<'d> Buzzer<'d> {
    pub fn new(
        io: impl Peripheral<P = impl InputPin + OutputPin> + 'd + 'static,
        ledc: esp_idf_hal::ledc::LEDC,
    ) -> Result<Self> {
        let config = config::TimerConfig::default().frequency(2000.Hz()).resolution(Resolution::Bits14);
        let mut timer = LedcTimerDriver::new(ledc.timer0, &config)?;
        let mut pwm = LedcDriver::new(
            ledc.channel0,
             &timer,
             io)?;

        let (tx, rx) = mpsc::channel::<BuzzerTone>();

        thread::Builder::new()
            .name("buzzer".into())
            .stack_size(4096)
            .spawn(move || {
                dbg!("Buzzer thread started");
                while let Ok(tone) = rx.recv() {
                    dbg!("Buzzer received tone: freq={}Hz, duration={}ms", tone.freq_hz, tone.duration_ms);
                    if tone.freq_hz == 0 || tone.duration_ms == 0 {
                        let _ = pwm.set_duty(0);
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                    timer.set_frequency(tone.freq_hz.Hz()).expect("Failed to set frequency");
                    let max_duty = pwm.get_max_duty();
                    let _ = pwm.set_duty(max_duty / 2);
                    dbg!("Buzzer playing tone: freq={}Hz, duration={}ms", tone.freq_hz, tone.duration_ms);
                    thread::sleep(Duration::from_millis(tone.duration_ms as u64));
                    dbg!("stop");
                    let _ = pwm.set_duty(0);
                }
                dbg!("Buzzer thread exiting");
            })?;

        Ok(Buzzer { sender: tx, _p: PhantomData })
    }

    pub fn enqueue_tones(&self, tones: &[BuzzerTone]) {
        for tone in tones {
            let _ = self.sender.send(BuzzerTone {
                freq_hz: tone.freq_hz,
                duration_ms: tone.duration_ms,
            });
        }
    }
}
