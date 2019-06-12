#![macro_use]

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
    }};
}



#[macro_export]
macro_rules! green {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Green, $fmt $(,$e)*);
        color_print!(White, "");
    };
}

#[macro_export]
macro_rules! red {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Red, $fmt $(,$e)*);
        color_print!(White, "");
    };
}

#[macro_export]
macro_rules! blue {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Blue, $fmt $(,$e)*);
        color_print!(White, "");
    };
}

#[macro_export]
macro_rules! yellow {
    ($fmt:expr $(,$e:expr)*) => {
        color_print!(Yellow, $fmt $(,$e)*);
        color_print!(White, "");
    };
}


#[macro_export]
macro_rules! flush {
    () => {{
        use std::io::Write;
        use std::io::stdout;
        match stdout().flush() {_=>{}};
    }}
}


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

