use std::{
    collections::HashMap,
    fmt::Display,
    io::Stdout,
    sync::Mutex,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::HorizontalAlignment,
    macros::constraints,
    style::{Color, Stylize},
    text::Span,
    widgets::{Block, Row, Table},
};

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

pub struct Logging {
    pub state: Mutex<LoggingState>,
}

pub struct LoggingState {
    pub start_time: Instant,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    coders_states: HashMap<u32, (Duration, CoderState)>,
    // dongles: Vec<DongleAvailability>,
}

enum CoderState {
    Compiling,
    Debugging,
    Refactoring,
    AcquiredDongle(u32),
    ReleasedDongle(u32),
    BurnedOut,
    Finished,
}

impl Logging {
    pub fn new() -> Self {
        let start_time = Instant::now();
        let terminal = ratatui::init();
        let state = Mutex::new(LoggingState {
            start_time,
            terminal,
            coders_states: HashMap::new(),
        });

        Self { state }
    }

    pub fn compile(&self, coder_id: u32) {
        {
            let LoggingState {
                start_time,
                coders_states,
                ..
            } = &mut *self.state.lock().unwrap();
            coders_states.insert(coder_id, (start_time.elapsed(), CoderState::Compiling));
        }
        self.log();
    }

    pub fn debug(&self, coder_id: u32) {
        {
            let LoggingState {
                start_time,
                coders_states,
                ..
            } = &mut *self.state.lock().unwrap();
            coders_states.insert(coder_id, (start_time.elapsed(), CoderState::Debugging));
        }
        self.log();
    }

    pub fn refactor(&self, coder_id: u32) {
        {
            let LoggingState {
                start_time,
                coders_states,
                ..
            } = &mut *self.state.lock().unwrap();
            coders_states.insert(coder_id, (start_time.elapsed(), CoderState::Refactoring));
        }
        self.log();
    }

    pub fn acquire(&self, coder_id: u32, dongle_id: u32) {
        {
            let LoggingState {
                start_time,
                coders_states,
                ..
            } = &mut *self.state.lock().unwrap();
            coders_states.insert(
                coder_id,
                (start_time.elapsed(), CoderState::AcquiredDongle(dongle_id)),
            );
        }
        self.log();
    }

    pub fn release(&self, coder_id: u32, dongle_id: u32) {
        {
            let LoggingState {
                start_time,
                coders_states,
                ..
            } = &mut *self.state.lock().unwrap();
            coders_states.insert(
                coder_id,
                (start_time.elapsed(), CoderState::ReleasedDongle(dongle_id)),
            );
        }
        self.log();
    }

    pub fn burnout(&self, coder_id: u32) {
        {
            let LoggingState {
                start_time,
                coders_states,
                ..
            } = &mut *self.state.lock().unwrap();
            coders_states.insert(coder_id, (start_time.elapsed(), CoderState::BurnedOut));
        }
        self.log();
    }

    pub fn finished(&self, coder_id: u32) {
        {
            let LoggingState {
                start_time,
                coders_states,
                ..
            } = &mut *self.state.lock().unwrap();
            coders_states.insert(coder_id, (start_time.elapsed(), CoderState::Finished));
        }
        self.log();
    }

    fn log(&self) {
        let LoggingState {
            terminal,
            coders_states,
            ..
        } = &mut *self.state.lock().unwrap();

        terminal
            .draw(|frame| {
                let mut rows: Vec<_> = coders_states.iter().collect();

                rows.sort_by_key(|(coder_id, _)| *coder_id);

                let rows = rows.iter().map(|&(&coder_id, (timestamp, state))| {
                    let timestamp = timestamp.as_millis().to_string();
                    let coder_name = CODER_NAMES[coder_id as usize % CODER_NAMES.len()].to_string();
                    let coder_color = Color::Indexed((coder_id % 256) as u8);

                    Row::new([timestamp.blue(), coder_name.fg(coder_color), state.styled()])
                });

                let header = Row::new(["time", "coder", "action"])
                    .underlined()
                    .bottom_margin(1);

                let block = Block::bordered()
                    .title("Codexion".bold())
                    .title_alignment(HorizontalAlignment::Center);

                let table = Table::new(rows, constraints![==20%, ==20%, >=20])
                    .header(header)
                    .block(block);

                frame.render_widget(table, frame.area())
            })
            .unwrap();
    }

    pub fn wait_for_exit(&self) {
        loop {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                ..
            })) = event::read()
            {
                break;
            }
        }
    }
}

impl CoderState {
    fn styled(&self) -> Span<'static> {
        let repr = self.to_string();

        match self {
            CoderState::Compiling => repr.green().bold(),
            CoderState::Debugging => repr.yellow().bold(),
            CoderState::Refactoring => repr.magenta().bold(),
            CoderState::AcquiredDongle(_) => repr.dim(),
            CoderState::ReleasedDongle(_) => repr.dim(),
            CoderState::BurnedOut => repr.red().bold(),
            CoderState::Finished => repr.cyan().on_blue().bold(),
        }
    }
}

impl Drop for Logging {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

impl Display for CoderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoderState::Compiling => write!(f, "COMPILING"),
            CoderState::Debugging => write!(f, "DEBUGGING"),
            CoderState::Refactoring => write!(f, "REFACTORING"),
            CoderState::AcquiredDongle(id) => write!(f, "ACQUIRED DONGLE({id})"),
            CoderState::ReleasedDongle(id) => write!(f, "RELEASED DONGLE({id})"),
            CoderState::BurnedOut => write!(f, "BURNED OUT"),
            CoderState::Finished => write!(f, "FINISHED"),
        }
    }
}
