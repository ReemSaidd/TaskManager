// https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
use crossterm::{
    event::{self, Event, KeyCode}
};
use std::{io};
use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use sysinfo::{System, SystemExt};
use unicode_width::UnicodeWidthStr;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::Command;


mod system;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    messages: Vec<String>,
    output: Vec<String>,
}

// impl Default for App {
//     fn default() -> App {
//         App {
//             input: String::new(),
//             input_mode: InputMode::Normal,
//             messages: Vec::new(),
//             output: Vec::new(),
//             sys: System::new_all(),
//         }
//     }
// }

impl App{
    pub fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn printptable(&mut self) -> i32 {
        let mut num: i32 = 0;
        let processes = psutil::process::processes().unwrap();
        self.output.push(format!("{:<10} {:<10} {:<20} {:<30}", "PID", "%CPU", "%MEM", "COMMAND"));
        for process in processes {
            let mut p = process.unwrap();
            match p.cmdline() {
                Ok(None) => {},
                _=> {num = num + 1;self.output.push(format!("{:<10} {:<10} {:<20} {:<30}", p.pid(), p.cpu_percent().unwrap(), p.memory_percent().unwrap(), p.cmdline().unwrap().expect("Oops something went wrong!").to_string()));},
            }
        }
        return num;
    }

    pub fn run_app<B: Backend>(&mut self,terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut flag: bool = false;
        let mut num: i32 = 0;
        let mut sys = System::new_all();
        let mut arg = String::new();
        let mut parts: Vec<String>;
        let mut history: Vec<String> = vec![];
        loop {
            terminal.draw(|f| self.ui(f))?;
            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Down => {
                            if flag {
                                if !(self.output.is_empty()) && history.len() <= (num-45).try_into().unwrap() {
                                    history.push(self.output.remove(1)); 
                                }                            
                            }
                        },
                        KeyCode::Up => {

                            if flag {
                                if !(history.is_empty()) {
                                    self.output.insert(1, history.pop().unwrap());
                                }                            
                            }
                        },
                        KeyCode::Enter => {
                            parts = self.input.split_whitespace().map(|s| s.to_string()).collect();
                            self.output.clear();
                            self.messages.push(self.input.drain(..).collect());
                            match parts[0].as_str() {
                                "uname" => {
                                    flag = false;
                                    self.output.push(format!("{}", sys.kernel_version().unwrap()))
                                },
                                "release" => {
                                    flag = false;
                                    self.output.push(format!("{}", sys.os_version().unwrap()))
                                },
                                "hostname" => {
                                    flag = false;
                                    self.output.push(format!("{}", sys.host_name().unwrap()))
                                },
                                "sysinfo" => {
                                    flag = false;
                                    self.output = system::get_system_information(&mut sys);
                                },
                                "sensors" => {
                                    flag = false;
                                    self.output = system::get_components_information(&mut sys);
                                },
                                "df" => {
                                    flag = false;
                                    if parts.len() == 2 {
                                        arg = parts[1][1..].to_string();
                                    }
                                    self.output = system::get_disks_information(&mut sys, arg.clone());
                                },
                                "hddtemp" => {
                                    flag = false;
                                    if parts.len() == 2 {
                                        arg = parts[1][1..].to_string();
                                    }
                                    self.output = system::get_hddtemp(&mut sys, arg.clone());
                                },
                                "lscpu" => {
                                    flag = false;
                                    self.output = system::get_cpu_information(&mut sys);
                                },
                                "gputemp" => {
                                    flag = false;
                                    if parts.len() == 2 {
                                        arg = parts[1][1..].to_string();
                                    }
                                    self.output = system::get_gputemp(&mut sys, arg.clone());
                                },
                                "kill" => {
                                    flag = false;
                                    if parts.len() == 2 {
                                        let pid = parts[1].parse::<i32>().unwrap();
                                        match kill(Pid::from_raw(pid), Signal::SIGTERM) {
                                            Ok(_) => self.output.push(format!("Process with killed successfully.\n")),
                                            Err(e) => self.output.push(format!("Error killing process: {}\n", e)),
                                        }
                                    }
                                },
                                "ignite" => {
                                    flag = false;
                                    if parts.len() == 2 {
                                        Command::new(parts[1].as_str()).output()?;    
                                    }
                                },
                                "ptable" => {
                                    num = self.printptable();
                                    flag = true;
                                }
                                "clear" => {
                                    flag = false;
                                    self.output.clear();
                                },

                                _ => {self.output.push(format!("{}: command not found\n", self.input))},
                            }
                            
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ],
                Style::default(),
            ),
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);

        let input = Paragraph::new(self.input.as_ref())
            .style(match self.input_mode {
                InputMode::Normal => Style::default().fg(Color::Yellow),
                InputMode::Editing => Style::default().fg(Color::Green),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, chunks[1]);
        match self.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            InputMode::Editing => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[1].x + self.input.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                )
            }
        }

        let output: Vec<ListItem> = self
            .output
            .iter()
            .enumerate()
            .map(|(_i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}", m)))];
                ListItem::new(content)
            })
            .collect();
        let output =
            List::new(output).block(Block::default().borders(Borders::ALL).title("Output")).style(Style::default().fg(Color::Green));


        
        f.render_widget(output, chunks[2]);
    }


}