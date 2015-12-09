use core::fmt;

#[derive(PartialEq, PartialOrd)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

fn write_nothing(_: &str) {}

static mut WRITE: fn(&str) = write_nothing;
static mut LEVEL: Level = Level::Info;

pub fn init(write: fn(&str), level: Level) {
    unsafe {
        WRITE = write;
        LEVEL = level;
    }
}

struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { WRITE(s); }
        Ok(())
    }
}

pub fn log(level: Level, args: fmt::Arguments) {
    unsafe {
        if level < LEVEL {
            return;
        }

        let writer: &mut fmt::Write = &mut Writer;
        let prefix = match level {
            Level::Debug => "d ",
            Level::Info => "i ",
            Level::Warning => "W ",
            Level::Error => "E ",
            Level::Fatal => "F ",
        };

        writer.write_str(prefix).unwrap();
        writer.write_fmt(args).unwrap();
        writer.write_str("\n").unwrap();
    }
}
