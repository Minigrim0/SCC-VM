use std::io;

use crossterm::{terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};


use crate::machine::Machine;
use crate::program::Program;

pub struct Decompiler {
    machine: Machine,
}


impl Decompiler {
    pub fn new() -> Self {
        Decompiler {
            machine: Machine::new(),
        }
    }

    /// Loads a binary file and 
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        let prog = Program::load(&path);
        self.machine = Machine::new();
        self.machine.load(prog);

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new("816 decompiler")
                .block(Block::default().title("Greeting").borders(Borders::ALL)),
            frame.size(),
        );
    }

    /// Runs the program interactively
    pub fn interactive(&mut self) -> Result<(), String> {
        if let Err(e) = enable_raw_mode() {
            return Err(e.to_string());
        };

        if let Err(e) = io::stdout().execute(EnterAlternateScreen) {
            return Err(e.to_string());
        }
        let mut terminal = match Terminal::new(CrosstermBackend::new(io::stdout())) {
            Ok(t) => t,
            Err(e) => return Err(e.to_string())
        };

        let result: Result<(), String> = loop {
            if let Err(e) = terminal.draw(|frame| self.draw(frame)) {
                break Err(e.to_string());
            }

            if let Ok(true) = event::poll(std::time::Duration::from_millis(50)) {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        match key.code {
                            KeyCode::Char('q') => break Ok(()),
                            KeyCode::Char('n') => {
                                match self.machine.step() {
                                    Ok(f) => {
                                        if f == true {
                                            break Ok(())
                                        }
                                    }
                                    Err(e) => break Err(format!("An error occured while running the machine: {}", e.to_string()))
                                }
                            },
                            _ => continue
                        }
                    }
                }
            }
        };

        if let Err(e) = disable_raw_mode() {
            return Err(e.to_string());
        };
        if let Err(e) = io::stdout().execute(LeaveAlternateScreen) {
            return Err(e.to_string());
        }

        result
    }

    /// Runs the program to completion, returning its output
    pub fn run(&mut self) -> Result<&String, String> {
        if let Err(e) = self.machine.run() {
            Err(format!("An error occured while running the machine: {}", e.to_string()))
        } else {
            Ok(self.machine.get_output())
        }
    }
}
