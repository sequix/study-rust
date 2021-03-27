// When attributes apply to a whole crate, their syntax is #![crate_attribute],
// When they apply to a module or item, the syntax is #[item_attribute].

#[allow(dead_code)]
fn noisy_unused_function() {}

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

// rustc --cfg some_condition custom.rs && ./custom
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

#[cfg(not(some_condition))]
fn conditional_function() {
    println!("condition not met!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    conditional_function();
}