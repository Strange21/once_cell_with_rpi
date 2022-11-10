
use pwm::{Channel, Polarity, Pwm};
use rppal::{
    gpio::{self, Gpio},
    pwm,
};
use std::time::Duration;
use std::thread;
use crate::init::{Motor,gpioPins};



const DIR_FORWARD: bool = false;
const DIR_BACKWARD: bool = true;



impl Motor for gpioPins{
    //  fn motor_stop(&mut self) {
    //     self.pwm_a.set_low();
    //     self.pwm_b.set_low();
    //     self.motor_a_pin1.set_low();
    //     self.motor_a_pin2.set_low();
    //     self.motor_b_pin1.set_low();
    //     self.motor_b_pin2.set_low();
    // }

    //  fn motor_right(&mut self, status: i32, direction: bool, speed: f64) {
    //     if status == 0 {
    //         self.motor_stop();
    //     } else {
    //         if direction == DIR_FORWARD {
    //             self.motor_b_pin1.set_high();
    //             self.motor_b_pin2.set_low();

    //             println!("motor right forwardat speed {:?}", speed);
    //             self.pwm_b.set_pwm(
    //                 Duration::from_millis(50),
    //                 Duration::from_millis(50),);
    //         } else {
    //             self.motor_b_pin1.set_low();
    //             self.motor_b_pin2.set_high();

    //             println!("motor right backwardat speed {:?}", speed);
    //             self.pwm_b.set_pwm(
    //                 Duration::from_millis(50),
    //                 Duration::from_millis(50),);
    //         }
    //     }
    // }

     fn motor_left(&mut self, status: i32, direction: bool, speed: f64) {
        if status == 0 {
            println!("motor stop");
            // self.motor_stop();
        } else {
            if direction == DIR_FORWARD {
                self.motor_a_pin1.set_high();
                self.motor_a_pin2.set_low();
                thread::sleep(Duration::from_secs(2));
                println!("motor left forward at speed {:?}", speed);
                self.pwm_a.set_pwm(
                    Duration::from_millis(50),
                    Duration::from_millis(50),);
            } else {
                self.motor_a_pin1.set_low();
                self.motor_a_pin2.set_high();

                println!("motor left backward at speed {:?}", speed);
                self.pwm_a.set_pwm(
                    Duration::from_millis(50),
                    Duration::from_millis(50),);
            }
        }
    }

    // fn motor_halt(&mut self) {
    //     self.motor_a_pin1.set_low();
    //     self.motor_a_pin2.set_low();
    //     self.motor_b_pin1.set_low();
    //     self.motor_b_pin2.set_low();
    // }
}