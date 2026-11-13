use crate::win;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

#[derive(Clone, Copy)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

impl Level {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Level::Debug,
            2 => Level::Warning,
            3 => Level::Error,
            _ => Level::Info,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Level::Debug => "Debug",
            Level::Info => "Info",
            Level::Warning => "Warning",
            Level::Error => "Error",
        }
    }

    fn color(self) -> u16 {
        match self {
            Level::Debug => 0x08,
            Level::Info => 0x07,
            Level::Warning => 0x0E,
            Level::Error => 0x0C,
        }
    }
}

struct State {
    path: String,
    console: win::HANDLE,
    backlog: Option<Vec<(Level, String, String)>>,
}

static STATE: Mutex<State> = Mutex::new(State { path: String::new(), console: 0, backlog: None });

pub fn init(path: &str, console: bool) {
    let _ = std::fs::write(path, "");
    let mut s = STATE.lock().unwrap();
    s.path = path.to_owned();
    s.backlog = if console { Some(Vec::new()) } else { None };
}

pub fn enable_console() {
    unsafe {
        win::AllocConsole();
        win::SetConsoleOutputCP(65001);
        win::SetConsoleTitleW(win::wide("Casualties Unknown - CUModLoader").as_ptr());
    }
    let input = win::open_console("CONIN$", win::GENERIC_READ | win::GENERIC_WRITE);
    if input != win::INVALID_HANDLE {
        let mut mode = 0;
        unsafe {
            win::GetConsoleMode(input, &mut mode);
            win::SetConsoleMode(input, (mode & !win::ENABLE_QUICK_EDIT_MODE) | win::ENABLE_EXTENDED_FLAGS);
        }
    }
    let out = win::open_console("CONOUT$", win::GENERIC_READ | win::GENERIC_WRITE);
    let mut s = STATE.lock().unwrap();
    if out == win::INVALID_HANDLE {
        s.backlog = None;
        return;
    }
    s.console = out;
    for (level, source, msg) in s.backlog.take().unwrap_or_default() {
        print_console(out, level, &source, &msg);
    }
}

pub fn write(level: Level, source: &str, msg: &str) {
    let mut t = win::SYSTEMTIME::default();
    unsafe { win::GetLocalTime(&mut t) };
    let line = format!(
        "[{:02}:{:02}:{:02}.{:03}] [{source}] [{}] {msg}\r\n",
        t.wHour,
        t.wMinute,
        t.wSecond,
        t.wMilliseconds,
        level.name()
    );
    let mut s = STATE.lock().unwrap();
    if !s.path.is_empty() {
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&s.path) {
            let _ = f.write_all(line.as_bytes());
        }
    }
    if s.console != 0 {
        print_console(s.console, level, source, msg);
    } else if let Some(b) = s.backlog.as_mut() {
        b.push((level, source.to_owned(), msg.to_owned()));
    }
}

fn print_console(h: win::HANDLE, level: Level, source: &str, msg: &str) {
    let head = format!("[{:<7}:{:>12}] ", level.name(), source);
    let body = format!("{}\r\n", msg.replace("\r\n", "\n").replace('\n', "\r\n"));
    unsafe {
        win::SetConsoleTextAttribute(h, level.color());
        put(h, &head);
        put(h, &body);
        win::SetConsoleTextAttribute(h, 0x07);
    }
}

unsafe fn put(h: win::HANDLE, s: &str) {
    let w: Vec<u16> = s.encode_utf16().collect();
    let mut n = 0;
    unsafe { win::WriteConsoleW(h, w.as_ptr(), w.len() as u32, &mut n, core::ptr::null_mut()) };
}

macro_rules! info {
    ($($t:tt)*) => { $crate::log::write($crate::log::Level::Info, "Native", &format!($($t)*)) };
}

macro_rules! error {
    ($($t:tt)*) => { $crate::log::write($crate::log::Level::Error, "Native", &format!($($t)*)) };
}

pub(crate) use {error, info};
