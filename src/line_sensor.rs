
use crate::init::{LineSensor,gpioPins};




impl LineSensor for gpioPins {

      fn run(&mut self) -> Vec<rppal::gpio::Level> {
        let mut sensor_list = vec![];

        let status_right = self.line_right.read();
        sensor_list.push(status_right);

        let status_left = self.line_left.read();
        sensor_list.push(status_left);

        let status_middle = self.line_middle.read();
        sensor_list.push(status_middle);

        sensor_list
    }
}
