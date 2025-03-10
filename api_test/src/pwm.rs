// pwm_blinkled.rs - Blinks an LED using hardware PWM.
//
// Remember to add a resistor of an appropriate value in series, to prevent
// exceeding the maximum current rating of the GPIO pin and the LED.
//
// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without disabling the PWM channel. Check out the
// gpio_blinkled_signals.rs example to learn how to properly handle incoming
// signals to prevent an abnormal termination.

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};

use std::sync::{Arc, Mutex};

pub fn set_pwm(pwm: Arc<Pwm>, intensity: Option<u32>) -> Result<(), Box<dyn Error>> {

    let intensity = intensity.unwrap_or(50);

    let intensity_pct: f64 = (intensity as f64) / 100.0;

    pwm.set_duty_cycle(intensity_pct).expect("could not set duty cycle");

    // Sleep for 50 milliseconds to avoid spamming the pwm
    thread::sleep(Duration::from_millis(50));

    Ok(())

    // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
    // You can manually disable the channel by calling the Pwm::disable() method.
}
