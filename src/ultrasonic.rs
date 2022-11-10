use rppal::gpio::{InputPin, OutputPin};
use std::thread;
use std::time::{Duration, Instant};
use crate::init::{Ultrasonic,gpioPins}; 

const ULTRA_SONIC_SPEED: f64 = 0.034; // cm /micro second

impl Ultrasonic for gpioPins{

  fn get_distance(&mut self) -> f64 {
        self.trig.set_low();
        thread::sleep(Duration::from_micros(2));
        self.trig.set_high();
        thread::sleep(Duration::from_micros(10));
        self.trig.set_low();
        let mut init = Instant::now();
        let mut start = Instant::now();
        let mut duration = Duration::new(0, 0);

        while self.echo.is_low() {
            start = Instant::now();
            if init.elapsed().as_millis() > 30 {
                return -1.0;
            }
        }

        init = Instant::now();
        while self.echo.is_high() {
            duration = start.elapsed();
            if init.elapsed().as_millis() > 30 {
                return -1.0;
            }
        }

        let micros = duration.as_micros();
        let distance = (ULTRA_SONIC_SPEED * micros as f64) / 2.0;
        return distance;
    }
}
