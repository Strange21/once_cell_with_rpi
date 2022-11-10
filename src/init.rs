
use std::cell::OnceCell;


use rppal::gpio::{self, Gpio, OutputPin,InputPin};

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

const LEFT_R: u8 = 22;
const LEFT_G: u8 = 23;
const LEFT_B: u8 = 24;

const RIGHT_R: u8 = 10;
const RIGHT_G: u8 = 9;
const RIGHT_B: u8 = 25;

const MOTOR_A_EN: u8 = 4;
const MOTOR_B_EN: u8 = 17;
const MOTOR_A_PIN1: u8 = 14;
const MOTOR_A_PIN2: u8 = 15;
const MOTOR_B_PIN1: u8 = 27;
const MOTOR_B_PIN2: u8 = 18;


const LINE_PIN_LEFT: u8 = 19;
const LINE_PIN_MIDDLE: u8 = 16;
const LINE_PIN_RIGHT: u8 = 20;

const GPIO_TRIG: u8 = 11;
const GPIO_ECHO: u8 = 8;

#[derive(Debug)]
pub struct GpioPins {
    pub left_r: OutputPin,
    pub left_g: OutputPin,
    pub left_b: OutputPin,
    pub right_r: OutputPin,
    pub right_g: OutputPin,
    pub right_b: OutputPin,
    pub motor_a_pin1: OutputPin,
    pub motor_a_pin2: OutputPin,
    pub motor_b_pin1: OutputPin,
    pub motor_b_pin2: OutputPin,
    pub pwm_a: OutputPin,
    pub pwm_b: OutputPin,
    pub line_right: InputPin,
    pub line_left: InputPin,
    pub line_middle: InputPin,
    pub trig: OutputPin,
    pub echo: InputPin,
}

static GPIO_INSTANCE: OnceCell<GpioPins> = OnceCell::new();

impl GpioPins
{
    pub fn global() -> &'static GpioPins {
        GLOBAL_DATA.get().expect("logger is not initialized")
    }

    pub fn initialization() -> GpioPins
    {
        let g1 = gpio::Gpio::new().unwrap();

        /*LED initialization */
        let left_r = g1.get(LEFT_R).unwrap().into_output();
        let left_g = g1.get(LEFT_G).unwrap().into_output();
        let left_b = g1.get(LEFT_B).unwrap().into_output();
        let right_r = g1.get(RIGHT_R).unwrap().into_output();
        let right_g = g1.get(RIGHT_G).unwrap().into_output();
        let right_b = g1.get(RIGHT_B).unwrap().into_output();

         /*MOTOR initialization */
        let motor_a_pin1 = g1.get(MOTOR_A_PIN1).unwrap().into_output();
        let motor_a_pin2 = g1.get(MOTOR_A_PIN2).unwrap().into_output();
        let motor_b_pin1 = g1.get(MOTOR_B_PIN1).unwrap().into_output();
        let motor_b_pin2 = g1.get(MOTOR_B_PIN2).unwrap().into_output();   
        let pwm_a = g1.get(MOTOR_A_EN).unwrap().into_output();
        let pwm_b = g1.get(MOTOR_B_EN).unwrap().into_output();

        /*LINE sensor initialization */
        let line_right = g1.get(LINE_PIN_RIGHT).unwrap().into_input();
        let line_left = g1.get(LINE_PIN_LEFT).unwrap().into_input();
        let line_middle = g1.get(LINE_PIN_MIDDLE).unwrap().into_input();

         /*Ultrasonic initialization */
        let  trig = g1.get(GPIO_TRIG).unwrap().into_output();
        let  echo = g1.get(GPIO_ECHO).unwrap().into_input();

        GpioPins { 
                    left_r, 
                    left_g,
                    left_b,
                    right_r,
                    right_g, 
                    right_b,
                    motor_a_pin1, 
                    motor_a_pin2, 
                    motor_b_pin1, 
                    motor_b_pin2, 
                    pwm_a, 
                    pwm_b,
                    line_right,
                    line_left,
                    line_middle,
                    trig,
                    echo }
        
    }
}


pub trait Led{
    fn both_on(&mut self);
    // fn both_off(&self);
    // fn side_on(&self,side_x: OutputPin);
    // fn side_off(&self,side_x: OutputPin);
    // fn blue(&self);
    // fn green(&self);
    // fn red(&self);
    // fn yellow(&self);
    // fn cyan(&self);
    // fn pink(&self);
    // fn side_color_on(&self,side_x: OutputPin,side_y:OutputPin);
    // fn side_color_off(&self,side_x: OutputPin,side_y:OutputPin);
    // fn turn_left(&mut self, times: i32);
    // fn turn_right(&mut self, times: i32);
    // fn police(&mut self, police_time: i32);
}

pub trait LineSensor{
    fn run(&mut self) -> Vec<rppal::gpio::Level> ;
}

pub trait Motor{
    // fn motor_stop(&mut self);
    // fn motor_right(&mut self, status: i32, direction: bool, speed: f64);
    fn motor_left(&mut self, status: i32, direction: bool, speed: f64);
    // fn motor_halt(&mut self);
}

pub trait Ultrasonic
{
    fn get_distance(&mut self) -> f64 ;
}