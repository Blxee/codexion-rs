use std::{io::Stdout, process::exit, sync::Mutex, time::Instant};

use ratatui::{
    Terminal, backend::CrosstermBackend, macros::ratatui_core::terminal, text::Line,
    widgets::Widget,
};

pub struct Logging {
    pub start_time_lock: Mutex<Instant>,
    pub terminal: Mutex<Terminal<CrosstermBackend<Stdout>>>,
}

impl Logging {
    pub fn new() -> Self {
        let start_time = Instant::now();
        let terminal = ratatui::init();

        Self {
            start_time_lock: Mutex::new(start_time),
            terminal: Mutex::new(terminal),
        }
    }

    pub fn compile(&self, coder_id: u32) {
        let start_time = *self.start_time_lock.lock().unwrap();
        let mut terminal = self.terminal.lock().unwrap();

        let line = Line::from(format!(
            "{}\t  COMPILING ",
            self.time_id_prefix(start_time, coder_id)
        ));
        terminal
            .draw(|frame| frame.render_widget(line, frame.area()))
            .unwrap();
    }

    pub fn debug(&self, coder_id: u32) {
        let start_time = *self.start_time_lock.lock().unwrap();
        let mut terminal = self.terminal.lock().unwrap();
        let line = Line::from(format!(
            "{}\t  DEBUGGING ",
            self.time_id_prefix(start_time, coder_id)
        ));
        terminal
            .draw(|frame| frame.render_widget(line, frame.area()))
            .unwrap();
    }

    pub fn refactor(&self, coder_id: u32) {
        let start_time = *self.start_time_lock.lock().unwrap();
        let mut terminal = self.terminal.lock().unwrap();
        let line = Line::from(format!(
            "{}\t REFACTORING",
            self.time_id_prefix(start_time, coder_id)
        ));
        terminal
            .draw(|frame| frame.render_widget(line, frame.area()))
            .unwrap();
    }

    pub fn acquire(&self, coder_id: u32, dongle_id: u32) {
        let start_time = *self.start_time_lock.lock().unwrap();
        let mut terminal = self.terminal.lock().unwrap();
        let line = Line::from(format!(
            "{}\t ACQUIRED dongle_{dongle_id}",
            self.time_id_prefix(start_time, coder_id)
        ));
        terminal
            .draw(|frame| frame.render_widget(line, frame.area()))
            .unwrap();
    }

    pub fn release(&self, coder_id: u32, dongle_id: u32) {
        let start_time = *self.start_time_lock.lock().unwrap();
        let mut terminal = self.terminal.lock().unwrap();
        let line = Line::from(format!(
            "{}\t RELEASED dongle_{dongle_id}",
            self.time_id_prefix(start_time, coder_id)
        ));
        terminal
            .draw(|frame| frame.render_widget(line, frame.area()))
            .unwrap();
    }

    pub fn burnout(&self, coder_id: u32) {
        let start_time = *self.start_time_lock.lock().unwrap();
        let mut terminal = self.terminal.lock().unwrap();
        let line = Line::from(format!(
            "{}\t  BURNED OUT ",
            self.time_id_prefix(start_time, coder_id)
        ));
        terminal
            .draw(|frame| frame.render_widget(line, frame.area()))
            .unwrap();
    }

    fn time_id_prefix(&self, start_time: Instant, coder_id: u32) -> String {
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

        let current_time = start_time.elapsed().as_millis();
        let coder_name = CODER_NAMES[coder_id as usize % CODER_NAMES.len()];

        format!("[{current_time:08}:{coder_name:^12}]")
    }
}

impl Drop for Logging {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
