use test_one_cell::init::Led;
use test_one_cell::init::Motor;
use test_one_cell::init;
use test_one_cell::motor;
use test_one_cell::led;


static GPIO_INSTANCE: OnceCell<GpioPins> = OnceCell::new();

fn main()->! {
    let gpio = GpioPins.initialization();
    
    unsafe{
        gpio_instance.both_on();
        gpio_instance.motor_left(1, false, 9.0);
        loop{}
    }
}
