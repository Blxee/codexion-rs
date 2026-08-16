use std::{io::Stdout, sync::Mutex, time::Instant};

use ratatui::{Terminal, backend::CrosstermBackend, text::Line};

pub struct Logging {
    pub state: Mutex<LoggingState>,
}
pub struct LoggingState {
    pub start_time: Instant,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Logging {
    pub fn new() -> Self {
        let start_time = Instant::now();
        let terminal = ratatui::init();
        let state = Mutex::new(LoggingState {
            start_time,
            terminal,
        });

        Self { state }
    }

    pub fn compile(&self, coder_id: u32) {
        let message = String::from("COMPILING");
        self.log(coder_id, message);
    }

    pub fn debug(&self, coder_id: u32) {
        let message = String::from("DEBUGGING");
        self.log(coder_id, message);
    }

    pub fn refactor(&self, coder_id: u32) {
        let message = String::from("REFACTORING");
        self.log(coder_id, message);
    }

    pub fn acquire(&self, coder_id: u32, dongle_id: u32) {
        let message = format!("ACQUIRED dongle_{dongle_id}",);
        self.log(coder_id, message);
    }

    pub fn release(&self, coder_id: u32, dongle_id: u32) {
        let message = format!("RELEASED dongle_{dongle_id}",);
        self.log(coder_id, message);
    }

    pub fn burnout(&self, coder_id: u32) {
        let message = String::from("COMPILING");
        self.log(coder_id, message);
    }

    fn log(&self, coder_id: u32, message: String) {
        const CODER_NAMES: [&'static str; 10] = [
            "otahiri",
            "abahoumi",
            "dsaouaf",
            "nait-sfi",
            "acherifi",
            "abourach",
            "atchioune",
            "oatiya",
            "aanouer",
            "atahiri",
        ];

        let LoggingState {
            start_time,
            terminal,
        } = &mut *self.state.lock().unwrap();

        let current_time = start_time.elapsed().as_millis();
        let coder_name = CODER_NAMES[coder_id as usize % CODER_NAMES.len()];
        let line = Line::from(format!("[{current_time:08}:{coder_name:^12}] {}", message));

        terminal
            .draw(|frame| frame.render_widget(line, frame.area()))
            .unwrap();
    }
}

impl Drop for Logging {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
