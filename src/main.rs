/// Nicholas Vuong - Sum of random numbers
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

use random_number::random;

/// Generate 2 randoms numbers and add them
fn random_add() -> u16 {
    /// Generate the 2 random numbers
    let num_1 = random!(..=10);
    let num_2 = random!(..=10);
    /// Print the numbers in the console
    println!("Number 1: {}", num_1);
    println!("Number 2: {}", num_2);
    /// Add the numbers
    let sum = num_1 + num_2;
    /// Print the sum in the console
    println!("Sum is: {}",sum);
    println!("");
    return sum;
}


/// Create the window
fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let sum = random_add();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(sum)
}

/// Create the UI
fn ui_builder() -> impl Widget<u16> {
    /// Create text for display
    let text = LocalizedString::new("hello-counter").with_arg("count", |sum: &u16, _env| (*sum).into());
    let label = Label::new(text).padding(5.0).center();
    /// Button the takes the sum of a new set of numbers
    let button = Button::new("Add")
        .on_click(|_ctx, sum, _env| {
            *sum = random_add()
        });
    
    /// Format the UI
    Flex::column().with_child(label).with_child(button)
}

