use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    simulated_expensive_calculation(1);

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let expensive_closure = |num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(simulated_user_specified_value as u64));
        num
    };

    println!("Hello, world!");
}

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: i32, random_number: i32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            )
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
