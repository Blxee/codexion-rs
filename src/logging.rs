use std::{sync::Mutex, time::Instant};

pub struct Logging {
    pub start_time_lock: Mutex<Instant>,
}

const RESET: &'static str = "\x1b[0m";

const BOLD: &'static str = "\x1b[1m";
const DIM: &'static str = "\x1b[2m";
const REVERSE: &'static str = "\x1b[7m";

const FG_BLACK: &'static str = "\x1b[30m";
const FG_RED: &'static str = "\x1b[31m";
const FG_GREEN: &'static str = "\x1b[32m";
const FG_YELLOW: &'static str = "\x1b[33m";
const FG_BLUE: &'static str = "\x1b[34m";
const FG_MAGENTA: &'static str = "\x1b[35m";
const FG_CYAN: &'static str = "\x1b[36m";
const FG_WHITE: &'static str = "\x1b[37m";

const BG_BLACK: &'static str = "\x1b[40m";
const BG_RED: &'static str = "\x1b[41m";
const BG_GREEN: &'static str = "\x1b[42m";
const BG_YELLOW: &'static str = "\x1b[43m";
const BG_BLUE: &'static str = "\x1b[44m";
const BG_MAGENTA: &'static str = "\x1b[45m";
const BG_CYAN: &'static str = "\x1b[46m";
const BG_WHITE: &'static str = "\x1b[47m";

impl Logging {
    pub fn new() -> Self {
        Self {
            start_time_lock: Mutex::new(Instant::now()),
        }
    }

    pub fn compile(&self, coder_id: u32) {
        let start_time = self.start_time_lock.lock().unwrap();
        println!(
            "{}\t {FG_CYAN}{BG_BLUE}{BOLD} COMPILING {RESET}",
            self.time_id_prefix(*start_time, coder_id)
        );
    }

    pub fn debug(&self, coder_id: u32) {
        let start_time = self.start_time_lock.lock().unwrap();
        println!(
            "{}\t {FG_YELLOW}{REVERSE} DEBUGGING {RESET}",
            self.time_id_prefix(*start_time, coder_id)
        );
    }

    pub fn refactor(&self, coder_id: u32) {
        let start_time = self.start_time_lock.lock().unwrap();
        println!(
            "{}\t {FG_MAGENTA}{REVERSE}REFACTORING{RESET}",
            self.time_id_prefix(*start_time, coder_id)
        );
    }

    pub fn acquire(&self, coder_id: u32, dongle_id: u32) {
        let start_time = self.start_time_lock.lock().unwrap();
        println!(
            "{}\t {DIM}ACQUIRED dongle_{dongle_id}{RESET}",
            self.time_id_prefix(*start_time, coder_id)
        );
    }

    pub fn release(&self, coder_id: u32, dongle_id: u32) {
        let start_time = self.start_time_lock.lock().unwrap();
        println!(
            "{}\t {DIM}RELEASED dongle_{dongle_id}{RESET}",
            self.time_id_prefix(*start_time, coder_id)
        );
    }

    pub fn burnout(&self, coder_id: u32) {
        let start_time = self.start_time_lock.lock().unwrap();
        println!(
            "{}\t {FG_BLACK}{BG_RED}{BOLD}BURNED OUT{RESET}",
            self.time_id_prefix(*start_time, coder_id)
        );
    }

    fn time_id_prefix(&self, start_time: Instant, coder_id: u32) -> String {
        const COLOR_CYCLE: [&'static str; 7] = [
            FG_RED, FG_GREEN, FG_YELLOW, FG_BLUE, FG_MAGENTA, FG_CYAN, FG_BLACK,
        ];

        let current_time = start_time.elapsed().as_millis();
        let coder_id_color = COLOR_CYCLE[coder_id as usize % COLOR_CYCLE.len()];

        format!(
            "{BOLD}[{FG_BLUE}{current_time:08}{RESET}{BOLD}:{FG_BLACK}{BG_WHITE}coder_{coder_id_color}{coder_id:03}{RESET}{BOLD}]{RESET}"
        )
    }
}
