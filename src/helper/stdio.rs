#![macro_use]
use std::io::{stdin, stdout, Write};



/// This function prompts the user with a message and returns the user's input.
/// It also pops off trailing carriage returns.
pub fn input<S: ToString>(prompt: S) -> String {
    let mut buf = String::new();
    print!("{}", prompt.to_string());
    let _ = stdout().flush();

    stdin().read_line(&mut buf).expect("Could not get user input");

    while let Some('\n') = buf.chars().next_back() {
        buf.pop();
    }

    while let Some('\r') = buf.chars().next_back() {
        buf.pop();
    }

    return buf;
}

/// Used to prompt the user with a yes or no question.
/// If they answer with Y or y, this function returns true.
pub fn yes_or_no<S: ToString>(prompt: S) -> bool {
    let response = input(prompt);

    response.to_lowercase().trim() == "y"
}



/// This prints a format string with a specific color.
/// The color must be one of the following.
/// - Black
/// - Blue
/// - Green
/// - Red
/// - Cyan
/// - Magenta
/// - Yellow
/// - White
/// THIS MUST BE CALLED AGAIN WITH `White` TO RESET THE COLOR AGAIN.
/// The color can be reset like so: `color_print!(White, "");`
#[macro_export]
macro_rules! color_print {
    ($color:ident, $fmt:expr $(,$e:expr)*) => {{
        use std::io::Write;
        use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

        let bufwtr = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();
        match buffer.set_color(ColorSpec::new().set_fg(Some(Color::$color))) {_=>{}};
        match write!(&mut buffer, $fmt $(,$e)*) {_=>{}};
        match bufwtr.print(&buffer) {_=>{}};
        match buffer.reset() {_=>{}};
        match bufwtr.print(&buffer) {_=>{}};
    }};
}


/// Write green text to the console, and then reset color
#[macro_export]
macro_rules! green {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Green, $fmt $(,$e)*);
        color_print!(White, "");
    };
}

/// Write red text to the console, and then reset color
#[macro_export]
macro_rules! red {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Red, $fmt $(,$e)*);
        color_print!(White, "");
    };
}

/// Write blue text to the console, and then reset color
#[macro_export]
macro_rules! blue {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Blue, $fmt $(,$e)*);
        color_print!(White, "");
    };
}

/// Write yellow text to the console, and then reset color
#[macro_export]
macro_rules! yellow {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Yellow, $fmt $(,$e)*);
        color_print!(White, "");
    };
}


/// Flush stdout
#[macro_export]
macro_rules! flush {
    () => {{
        use std::io::Write;
        use std::io::stdout;
        match stdout().flush() {_=>{}};
    }}
}


/// Prints info message colored green
#[macro_export]
macro_rules! info {
    ($fmt:expr $(,$e:expr)*) => {
        let user = format!($fmt $(, $e)*);
        print!("==[");
        flush!();
        green!("INFO{}", "");
        print!("]===> {}\n", user);
    };
}

/// Prints debug message colored blue
#[macro_export]
macro_rules! debug {
    ($fmt:expr $(,$e:expr)*) => {
        let user = format!($fmt $(, $e)*);
        print!("==[");
        flush!();
        blue!("DEBUG");
        print!("]==> {}\n", user);
    };
}

/// Prints error message colored red
#[macro_export]
macro_rules! error {
    ($fmt:expr $(,$e:expr)*) => {
        let user = format!($fmt $(, $e)*);
        print!("==[");
        flush!();
        red!("ERROR");
        print!("]==> {}\n", user);
    };
}


/// Prints warning message colored yellow
#[macro_export]
macro_rules! warn {
    ($fmt:expr $(,$e:expr)*) => {
        let user = format!($fmt $(, $e)*);
        print!("==[");
        flush!();
        yellow!("WARN");
        print!("]===> {}\n", user);
    };
}

