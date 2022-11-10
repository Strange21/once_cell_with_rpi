use std::thread;
use std::time::Duration;
use crate::init::{Led,gpioPins};
use rppal::gpio::OutputPin;

impl Led for gpioPins {
    fn both_on(&mut self) {
        self.left_r.set_high();
        self.left_g.set_high();
        self.left_b.set_high();

        self.right_r.set_low();
        self.right_g.set_low();
        self.right_b.set_low();

        thread::sleep(Duration::from_secs(2));
    }

    
}



// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_LED: u8 = 23;

// const LEFT_R: u8 = 22;
// const LEFT_G: u8 = 23;
// const LEFT_B: u8 = 24;

// const RIGHT_R: u8 = 10;
// const RIGHT_G: u8 = 9;
// const RIGHT_B: u8 = 25;

// pub struct Led {
//     pub left_r: rppal::gpio::OutputPin,
//     pub left_g: rppal::gpio::OutputPin,
//     pub left_b: rppal::gpio::OutputPin,
//     pub right_r: rppal::gpio::OutputPin,
//     pub right_g: rppal::gpio::OutputPin,
//     pub right_b: rppal::gpio::OutputPin,
// }


// impl Led {
//     pub fn new() -> Led {
//         let left_r1 = gpio::Gpio::new()
//             .unwrap()
//             .get(LEFT_R)
//             .unwrap()
//             .into_output();
//         let left_g1 = gpio::Gpio::new()
//             .unwrap()
//             .get(LEFT_G)
//             .unwrap()
//             .into_output();
//         let left_b1 = gpio::Gpio::new()
//             .unwrap()
//             .get(LEFT_B)
//             .unwrap()
//             .into_output();
//         let right_r1 = gpio::Gpio::new()
//             .unwrap()
//             .get(RIGHT_R)
//             .unwrap()
//             .into_output();
//         let right_g1 = gpio::Gpio::new()
//             .unwrap()
//             .get(RIGHT_G)
//             .unwrap()
//             .into_output();
//         let right_b1 = gpio::Gpio::new()
//             .unwrap()
//             .get(RIGHT_B)
//             .unwrap()
//             .into_output();

//         Led {
//             left_r: left_r1,
//             left_g: left_g1,
//             left_b: left_b1,
//             right_r: right_r1,
//             right_g: right_g1,
//             right_b: right_b1,
//         }
//     }

//     pub fn both_on(&mut self) {
//         self.left_r.set_low();
//         self.left_g.set_low();
//         self.left_b.set_low();

//         self.right_r.set_low();
//         self.right_g.set_low();
//         self.right_b.set_low();
//     }

//     pub fn both_off(&mut self) {
//         self.left_r.set_high();
//         self.left_g.set_high();
//         self.left_b.set_high();

//         self.right_r.set_high();
//         self.right_g.set_high();
//         self.right_b.set_high();
//     }

//     pub fn side_on(&mut self, side_x: u8) {
//         let new_gpio = gpio::Gpio::new();
//         match new_gpio {
//             Ok(gpio) => {
//                 let new_pin = gpio.get(side_x);
//                 match new_pin {
//                     Ok(pin) => {
//                         let mut out_pin = pin.into_output();
//                         out_pin.set_low();
//                     }
//                     Err(_) => {}
//                 }
//             }
//             Err(_) => {}
//         }
//     }

//     pub fn side_off(&mut self, side_x: u8) {
//         let mut mypin = gpio::Gpio::new()
//             .unwrap()
//             .get(side_x)
//             .unwrap()
//             .into_output();
//         mypin.set_high();
//     }

//     pub fn police(&mut self, police_time: i32) {
//         for _i in 1..police_time {
//             for _j in 1..3 {
//                 self.side_on(LEFT_R);
//                 self.side_on(RIGHT_B);
//                 thread::sleep(Duration::from_millis(100));
//                 self.both_off();
//                 self.side_on(LEFT_B);
//                 self.side_on(RIGHT_R);
//                 thread::sleep(Duration::from_millis(100));
//                 self.both_off();
//             }

//             for _j in 1..5 {
//                 self.side_on(LEFT_R);
//                 self.side_on(RIGHT_B);
//                 thread::sleep(Duration::from_millis(100));
//                 self.both_off();
//                 self.side_on(LEFT_B);
//                 self.side_on(RIGHT_R);
//                 thread::sleep(Duration::from_millis(100));
//                 self.both_off();
//             }
//         }
//     }

//     pub fn red(&mut self) {
//         self.side_on(RIGHT_R);
//         self.side_on(LEFT_R);
//     }

//     pub fn green(&mut self) {
//         self.side_on(RIGHT_G);
//         self.side_on(LEFT_G);
//     }

//     pub fn blue(&mut self) {
//         self.side_on(RIGHT_B);
//         self.side_on(LEFT_B);
//     }

//     pub fn yellow(&mut self) {
//         self.red();
//         self.green();
//     }

//     pub fn pink(&mut self) {
//         self.red();
//         self.blue();
//     }

//     pub fn cyan(&mut self) {
//         self.blue();
//         self.green();
//     }

//     pub fn side_color_on(&mut self, mut side_x: OutputPin, mut side_y: OutputPin) {
//         side_x.set_high();
//         side_y.set_high();
//     }

//     pub fn side_color_off(&mut self, mut side_x: OutputPin, mut side_y: OutputPin) {
//         side_x.set_low();
//         side_y.set_low();
//     }

//     pub fn turn_left(&mut self, times: i32) {
//         for _i in 0..times {
//             self.both_off();
//             self.side_on(LEFT_G);
//             self.side_on(LEFT_R);
//             thread::sleep(Duration::from_millis(500));
//             self.both_off();
//             thread::sleep(Duration::from_millis(500));
//         }
//     }

//     pub fn turn_right(&mut self, times: i32) {
//         for _i in 1..times {
//             self.both_off();
//             self.side_on(RIGHT_G);
//             self.side_on(RIGHT_R);
//             thread::sleep(Duration::from_millis(500));
//             self.both_off();
//             thread::sleep(Duration::from_millis(500));
//         }
//     }
// }

// fn main() -> Result<(), Box<dyn Error>> Ok({
//     let l1 = Led::new();
//     l1.both_on();

// if __name__ == '__main__':
//     setup()
//     police(4)
//     both_on()
//     time.sleep(1)
//     both_off()
//     yellow()
//     time.sleep(5)
//     both_off()
//     pink()
//     time.sleep(5)
//     both_off()
//     cyan()
//     time.sleep(5)
//     both_off()
//}
