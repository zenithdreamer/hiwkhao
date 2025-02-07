use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::vm::VM;

pub fn run_tui(vm: Arc<Mutex<VM>>) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let res = run_app(&mut terminal, vm);

    // Restore terminal
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, vm: Arc<Mutex<VM>>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &vm))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('s') => {
                        let mut vm = vm.lock().unwrap();
                        vm.step();
                    }
                    KeyCode::Char('r') => {
                        let mut vm = vm.lock().unwrap();
                        vm.run();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, vm: &Arc<Mutex<VM>>) {
    let size = f.size();
    let vm = vm.lock().unwrap();
    let (registers, memory, pc) = vm.get_state();
    let program = vm.get_program();

    // Create blocks
    let registers_block = Block::default()
        .title("Registers")
        .borders(Borders::ALL);
    let program_block = Block::default()
        .title("Program")
        .borders(Borders::ALL);
    let memory_block = Block::default()
        .title("Memory")
        .borders(Borders::ALL);

    // Split screen into sections
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(size);

    // Create register text
    let mut reg_text = Vec::new();
    for (reg, value) in registers {
        reg_text.push(Line::from(vec![Span::raw(format!("{}: {}", reg, value))]));
    }
    reg_text.sort_by(|a, b| {
        let a_str = a.spans[0].content.as_ref();
        let b_str = b.spans[0].content.as_ref();
        a_str.cmp(b_str)
    });

    // Create program text with current instruction highlighted
    let mut program_text = Vec::new();
    for (i, instruction) in program.iter().enumerate() {
        let line = if i == pc {
            Line::from(vec![Span::styled(
                format!("> {}", instruction),
                Style::default().fg(Color::Yellow),
            )])
        } else {
            Line::from(vec![Span::raw(format!("  {}", instruction))])
        };
        program_text.push(line);
    }

    // Create memory text
    let mut memory_text = Vec::new();
    for i in (0..memory.len()).step_by(4) {
        if i + 3 < memory.len() {
            let value = i32::from_le_bytes([
                memory[i],
                memory[i + 1],
                memory[i + 2],
                memory[i + 3],
            ]);
            if value != 0 {
                memory_text.push(Line::from(vec![Span::raw(format!("{:#04x}: {}", i, value))]));
            }
        }
    }

    // Render widgets
    f.render_widget(
        Paragraph::new(reg_text).block(registers_block),
        chunks[0],
    );
    f.render_widget(
        Paragraph::new(program_text).block(program_block),
        chunks[1],
    );
    f.render_widget(
        Paragraph::new(memory_text).block(memory_block),
        chunks[2],
    );
}
