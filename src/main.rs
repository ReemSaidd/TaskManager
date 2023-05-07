// https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use sysinfo::{ComponentExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt, CpuRefreshKind, RefreshKind, DiskExt};
use unicode_width::UnicodeWidthStr;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::str;
use std::process::Command;

use psutil::process::{Process, ProcessError};

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    messages: Vec<String>,
    output: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            output: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut flag: bool = false;
    let mut num: i32 = 0;
    let mut sys = System::new_all();
    let mut arg = String::new();
    let mut parts: Vec<String>;
    let mut history: Vec<String> = vec![];
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Down => {
                        if flag {
                            if !(app.output.is_empty()) && history.len() <= (num-45).try_into().unwrap() {
                                history.push(app.output.remove(1)); 
                            }                            
                        }
                    },
                    KeyCode::Up => {

                        if flag {
                            if !(history.is_empty()) {
                                app.output.insert(1, history.pop().unwrap());
                            }                            
                        }
                    },
                    KeyCode::Enter => {
                        parts = app.input.split_whitespace().map(|s| s.to_string()).collect();
                        app.output.clear();
                        app.messages.push(app.input.drain(..).collect());
                        match parts[0].as_str() {
                            "uname" => {
                                flag = false;
                                app.output.push(format!("{}", sys.kernel_version().unwrap()))
                            },
                            "release" => {
                                flag = false;
                                app.output.push(format!("{}", sys.os_version().unwrap()))
                            },
                            "hostname" => {
                                flag = false;
                                app.output.push(format!("{}", sys.host_name().unwrap()))
                            },
                            "sysinfo" => {
                                flag = false;
                                app.output = get_system_information(&mut sys);
                            },
                            "sensors" => {
                                flag = false;
                                app.output = get_components_information(&mut sys);
                            },
                            "df" => {
                                flag = false;
                                if parts.len() == 2 {
                                    arg = parts[1][1..].to_string();
                                }
                                app.output = get_disks_information(&mut sys, arg.clone());
                            },
                            "hddtemp" => {
                                flag = false;
                                if parts.len() == 2 {
                                    arg = parts[1][1..].to_string();
                                }
                                app.output = get_hddtemp(&mut sys, arg.clone());
                            },
                            "lscpu" => {
                                flag = false;
                                app.output = get_cpu_information(&mut sys);
                            },
                            "gputemp" => {
                                flag = false;
                                if parts.len() == 2 {
                                    arg = parts[1][1..].to_string();
                                }
                                app.output = get_gputemp(&mut sys, arg.clone());
                            },
                            "kill" => {
                                flag = false;
                                if parts.len() == 2 {
                                    let pid = parts[1].parse::<i32>().unwrap();
                                    match kill(Pid::from_raw(pid), Signal::SIGTERM) {
                                        Ok(_) => app.output.push(format!("Process with killed successfully.\n")),
                                        Err(e) => app.output.push(format!("Error killing process: {}\n", e)),
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
                                num = printptable(&mut app);
                                flag = true;
                            }
                            "clear" => {
                                flag = false;
                                app.output.clear();
                            },

                            _ => {app.output.push(format!("{}: command not found\n", app.input))},
                        }
                        
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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

    let (msg, style) = match app.input_mode {
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

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Yellow),
            InputMode::Editing => Style::default().fg(Color::Green),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let output: Vec<ListItem> = app
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


fn get_system_information(sys: &System) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    vec.push(format!("Name: {}", sys.name().unwrap()));
    vec.push(format!("Kernel version: {}", sys.kernel_version().unwrap()));
    vec.push(format!("OS version: {}", sys.os_version().unwrap()));
    vec.push(format!("Host name: {}", sys.host_name().unwrap()));
    return vec;
}

fn get_components_information(sys: &mut System) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    for component in sys.components() {
        vec.push(format!("{:?}", component));
    }
    return vec;
}

fn get_hddtemp(sys: &mut System, arg: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    match arg.as_str() {
        "" => {
            for component in sys.components_mut() {
                if component.label().contains("SSD") || component.label().contains("HDD"){
                    vec.push(format!("{}: {:?}°C", component.label(), component.temperature()));
                    component.refresh();
                }
            }            
        },
        "max" => {
            for component in sys.components_mut() {
                if component.label().contains("SSD") || component.label().contains("HDD"){
                    vec.push(format!("{}: {:?}°C", component.label(), component.max()));
                    component.refresh();
                }
            }
        },
        "crit" => {
            for component in sys.components_mut() {
                if component.label().contains("SSD") || component.label().contains("HDD"){
                    vec.push(format!("{}: {:?}°C", component.label(), component.critical().unwrap()));
                    component.refresh();
                }
            }
        },
        _ => {},
    }   
    return vec;
}

fn get_disks_information(sys: &mut System, arg: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let base: u64 = 2;
    let mut power: u32 = 0;
    match arg.as_str() {
        "" => {power = 0;},
        "k" => {power = 10;},
        "m" => {power = 20;},
        _ => {},
    }
    vec.push(format!("{:<50} {:<50} {:<50} {:<50} {:<50} {:<50}", "Name", "Mount Point", "Filesystem", "Total Space", "Available Space", "Used Space"));
    for disk in sys.disks() {
        vec.push(format!("{:<50} {:<50} {:<50} {:<50} {:<50} {:<50}", disk.name().to_str().unwrap(), disk.mount_point().to_str().unwrap(), str::from_utf8(disk.file_system()).unwrap(), disk.total_space()/(base.pow(power)), disk.available_space()/(base.pow(power)), disk.total_space()/(base.pow(power)) - disk.available_space()/(base.pow(power))));
    }
    return vec;
}

fn get_cpu_information(sys: &mut System) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    vec.push(format!("{:<50} {:<50} {:<50} {:<50}", "Brand", "Vendor ID", "Name", "Frequency"));
    for cpu in sys.cpus() {
        vec.push(format!("{:<50} {:<50} {:<50} {:<50}", cpu.brand(), cpu.vendor_id(), cpu.name(), cpu.frequency()));
    }
    return vec;
}

fn get_gputemp(sys: &mut System, arg: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    match arg.as_str() {
        "" =>  {
            for component in sys.components_mut() {
                if component.label().contains("gpu") {
                    vec.push(format!("{}: {}°C", component.label(), component.temperature()));
                    component.refresh();
                }
            }       
        },
        "max" => {
            for component in sys.components_mut() {
                if component.label().contains("gpu"){
                    vec.push(format!("{}: {}°C", component.label(), component.max()));
                    component.refresh();
                }
            }   
        },
        _ => {}
    }
    return vec;
}

fn printptable(app: &mut App) -> i32 {
    let mut num: i32 = 0;
    let processes = psutil::process::processes().unwrap();
    app.output.push(format!("{:<10} {:<10} {:<20} {:<30}", "PID", "%CPU", "%MEM", "COMMAND"));
    for process in processes {
        let mut p = process.unwrap();
        match p.cmdline() {
            Ok(None) => {},
            _=> {num = num + 1;app.output.push(format!("{:<10} {:<10} {:<20} {:<30}", p.pid(), p.cpu_percent().unwrap(), p.memory_percent().unwrap(), p.cmdline().unwrap().expect("Oops something went wrong!").to_string()));},
        }
    }
    return num;
}
