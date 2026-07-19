#![allow(dead_code, unused_variables)]
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use std::process;

// --- ASSINATURAS NATIVAS DO SISTEMA (Substitui a crate libc externa) ---
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

const TIOCGWINSZ: u64 = 0x5413; // Valor padrão para Linux x86_64
const TCSAFLUSH: i32 = 2;

const BRKINT: u32 = 0o000002;
const ICRNL: u32 = 0o000400;
const INPCK: u32 = 0o000020;
const ISTRIP: u32 = 0o000040;
const IXON: u32 = 0o002000;
const OPOST: u32 = 0o000001;
const CS8: u32 = 0o000060;
const ECHO: u32 = 0o000010;
const ICANON: u32 = 0o000002;
const IEXTEN: u32 = 0o040000;
const ISIG: u32 = 0o000001;
const VMIN: usize = 6;
const VTIME: usize = 5;

extern "C" {
    fn tcgetattr(fd: i32, termios_p: *mut termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const termios) -> i32;
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}
// ----------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
enum Key {
    Char(char),
    Ctrl(char),
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    PageUp,
    PageDown,
    Home,
    End,
    Delete,
    Escape,
}

struct Row {
    chars: String,
    render: String,
}

struct EditorConfig {
    cx: usize,
    cy: usize,
    screen_rows: usize,
    screen_cols: usize,
    rows: Vec<Row>,
}

impl EditorConfig {
    fn new() -> Self {
        let mut config = Self {
            cx: 0,
            cy: 0,
            screen_rows: 0,
            screen_cols: 0,
            rows: Vec::new(),
        };
        config.get_window_size();
        config
    }

    fn get_window_size(&mut self) {
        let mut ws = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        
        unsafe {
            if ioctl(1, TIOCGWINSZ, &mut ws) == -1 || ws.ws_col == 0 {
                self.screen_rows = 24;
                self.screen_cols = 80;
                return;
            }
        }
        self.screen_rows = ws.ws_row as usize;
        self.screen_cols = ws.ws_col as usize;
    }
}

struct RawMode {
    orig_termios: termios,
}

impl RawMode {
    fn enable() -> io::Result<Self> {
        let fd = io::stdin().as_raw_fd();
        let mut t = unsafe {
            let mut raw_t = std::mem::zeroed();
            if tcgetattr(fd, &mut raw_t) == -1 {
                return Err(io::Error::last_os_error());
            }
            raw_t
        };

        let orig_termios = t;

        t.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        t.c_oflag &= !(OPOST);
        t.c_cflag |= CS8;
        t.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        t.c_cc[VMIN] = 0;
        t.c_cc[VTIME] = 1;

        unsafe {
            if tcsetattr(fd, TCSAFLUSH, &t) == -1 {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(RawMode { orig_termios })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let fd = io::stdin().as_raw_fd();
        unsafe {
            tcsetattr(fd, TCSAFLUSH, &self.orig_termios);
            print!("\x1b[2J\x1b[H");
            let _ = io::stdout().flush();
        }
    }
}

fn editor_read_key() -> Key {
    let mut stdin = io::stdin();
    let mut buffer = [0; 1];

    loop {
        match stdin.read(&mut buffer) {
            Ok(0) => continue,
            Ok(_) => {
                let c = buffer[0];
                if c == 27 {
                    let mut seq = [0; 3];
                    if stdin.read(&mut seq[0..1]).unwrap_or(0) == 0 { return Key::Escape; }
                    if stdin.read(&mut seq[1..2]).unwrap_or(0) == 0 { return Key::Escape; }

                    if seq[0] == b'[' {
                        if seq[1] >= b'0' && seq[1] <= b'9' {
                            if stdin.read(&mut seq[2..3]).unwrap_or(0) == 0 { return Key::Escape; }
                            if seq[2] == b'~' {
                                match seq[1] {
                                    b'3' => return Key::Delete,
                                    b'5' => return Key::PageUp,
                                    b'6' => return Key::PageDown,
                                    _ => {}
                                }
                            }
                        } else {
                            match seq[1] {
                                b'A' => return Key::ArrowUp,
                                b'B' => return Key::ArrowDown,
                                b'C' => return Key::ArrowRight,
                                b'D' => return Key::ArrowLeft,
                                b'H' => return Key::Home,
                                b'F' => return Key::End,
                                _ => {}
                            }
                        }
                    } else if seq[0] == b'O' {
                        match seq[1] {
                            b'H' => return Key::Home,
                            b'F' => return Key::End,
                            _ => {}
                        }
                    }
                    return Key::Escape;
                } else if c == 127 {
                    return Key::Backspace;
                } else if c > 0 && c < 32 {
                    return Key::Ctrl((c + b'@') as char);
                } else {
                    return Key::Char(c as char);
                }
            }
            Err(_) => process::exit(1),
        }
    }
}

fn editor_refresh_screen(config: &EditorConfig) {
    let mut ab = String::new();
    ab.push_str("\x1b[?25l");
    ab.push_str("\x1b[H");

    for y in 0..config.screen_rows {
        if y == config.screen_rows / 3 && config.rows.is_empty() {
            let mut welcome = format!("Kilo editor -- version Rust (Nativo) 0.0.1");
            welcome.truncate(config.screen_cols);
            let mut padding = (config.screen_cols - welcome.len()) / 2;
            if padding > 0 {
                ab.push('~');
                padding -= 1;
            }
            ab.push_str(&" ".repeat(padding));
            ab.push_str(&welcome);
        } else {
            ab.push_str("~");
        }
        ab.push_str("\x1b[0K");
        if y < config.screen_rows - 1 {
            ab.push_str("\r\n");
        }
    }

    ab.push_str(&format!("\x1b[{};{}H", config.cy + 1, config.cx + 1));
    ab.push_str("\x1b[?25h");

    print!("{}", ab);
    let _ = io::stdout().flush();
}

fn editor_process_keypress() -> bool {
    let key = editor_read_key();
    match key {
        Key::Ctrl('Q') => false,
        _ => true,
    }
}

fn main() {
    let _raw_mode = RawMode::enable().expect("Falha ao iniciar Raw Mode");
    let config = EditorConfig::new();

    loop {
        editor_refresh_screen(&config);
        if !editor_process_keypress() {
            break;
        }
    }
}
