use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};
use iced::{alignment, theme, Background, Color, Element, Length, Application, Settings, Command};
use iced::widget::container::Appearance;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::vm::VM;

// Color scheme
const BG_COLOR: Color = Color::from_rgb(1.0, 1.0, 1.0);        // White background
const DARKER_BG: Color = Color::from_rgb(0.95, 0.95, 0.95);   // Light gray for contrast
const TEXT_COLOR: Color = Color::from_rgb(0.1, 0.1, 0.1);     // Almost black text
const HIGHLIGHT_COLOR: Color = Color::from_rgb(0.0, 0.47, 0.95); // Bright blue highlight
const MUTED_TEXT: Color = Color::from_rgb(0.4, 0.4, 0.4);     // Gray text
const BORDER_COLOR: Color = Color::from_rgb(0.8, 0.8, 0.8);   // Light gray borders
const CURRENT_LINE_BG: Color = Color::from_rgb(0.9, 0.95, 1.0); // Light blue background for current line

struct CustomContainer;
struct HighlightContainer;
struct ContentContainer;

impl container::StyleSheet for CustomContainer {
    type Style = theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(TEXT_COLOR),
            background: Some(Background::Color(BG_COLOR)),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
        }
    }
}

impl container::StyleSheet for HighlightContainer {
    type Style = theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(TEXT_COLOR),
            background: Some(Background::Color(HIGHLIGHT_COLOR)),
            border_radius: 8.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl container::StyleSheet for ContentContainer {
    type Style = theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(TEXT_COLOR),
            background: Some(Background::Color(CURRENT_LINE_BG)),
            border_radius: 6.0.into(),
            border_width: 1.0,
            border_color: BORDER_COLOR,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Step,
    Run,
    Reset,
    ToggleBreakpoint(usize),
    Continue,
    UpdateMemoryStart(String),
    ClearOutput,
    AddOutput(String),
    ScrollTo(f32),
}

pub struct EmulatorGui {
    vm: Arc<Mutex<VM>>,
    program_text: String,
    output: Vec<String>,
    breakpoints: HashSet<usize>,
    memory_start_addr: String,
    is_running: bool,
}

impl EmulatorGui {
    fn format_memory_view(&self, start_addr: usize) -> String {
        let mut output = String::new();
        if let Ok(vm) = self.vm.lock() {
            let (_, memory, _) = vm.get_state();
            
            // Header
            output.push_str("Address  | 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F | ASCII\n");
            output.push_str("─────────┼─────────────────────────────────────────────────┼─────────────\n");
            
            // Show all memory in 16-byte chunks
            for i in (0..memory.len()).step_by(16) {
                // Address
                output.push_str(&format!("{:08X} │ ", i));
                
                // Hex values
                for j in 0..16 {
                    if i + j < memory.len() {
                        output.push_str(&format!("{:02X} ", memory[i + j]));
                    } else {
                        output.push_str("   ");
                    }
                }
                
                // ASCII representation
                output.push_str("│ ");
                for j in 0..16 {
                    if i + j < memory.len() {
                        let byte = memory[i + j];
                        if byte >= 32 && byte <= 126 {
                            output.push(byte as char);
                        } else {
                            output.push('.');
                        }
                    } else {
                        output.push(' ');
                    }
                }
                output.push('\n');
            }
        }
        output
    }

    fn format_registers(&self) -> String {
        let mut output = String::new();
        if let Ok(vm) = self.vm.lock() {
            let (registers, _, _) = vm.get_state();
            // Convert to vec for sorting
            let mut reg_pairs: Vec<_> = registers.iter().collect();
            // Sort by register name
            reg_pairs.sort_by(|(a, _), (b, _)| {
                // Extract numeric part of register name (e.g., "R1" -> 1)
                let a_num = a.trim_start_matches('R').parse::<i32>().unwrap_or(0);
                let b_num = b.trim_start_matches('R').parse::<i32>().unwrap_or(0);
                a_num.cmp(&b_num)
            });
            // Format sorted registers
            for (reg, value) in reg_pairs {
                output.push_str(&format!("{}: {}\n", reg, value));
            }
        }
        output
    }

    fn log_state(&self) {
        if let Ok(vm) = self.vm.lock() {
            let (registers, memory, pc) = vm.get_state();
            println!("\n=== VM State ===");
            println!("PC: {}", pc);
            println!("Registers:");
            for (reg, value) in registers {
                println!("  {}: {}", reg, value);
            }
            println!("Memory (first 64 bytes):");
            for i in (0..64.min(memory.len())).step_by(16) {
                print!("{:08x}:", i);
                for j in 0..16 {
                    if i + j < memory.len() {
                        print!(" {:02x}", memory[i + j]);
                    }
                }
                println!();
            }
            println!("===============\n");
        }
    }

    fn instruction_rows(&self) -> (Vec<Element<Message>>, Option<usize>) {
        let mut rows = Vec::new();
        let mut current_line_idx = None;
        if let Ok(vm) = self.vm.lock() {
            let (_, _, current_pc) = vm.get_state();
            for (i, line) in self.program_text.lines().enumerate() {
                let is_current = i == current_pc;
                if is_current {
                    current_line_idx = Some(i);
                }
                let has_breakpoint = self.breakpoints.contains(&i);
                
                let line_number = text(format!("{:4} ", i))
                    .size(14)
                    .style(theme::Text::Color(MUTED_TEXT));
                
                let breakpoint = checkbox(
                    "",
                    has_breakpoint,
                    move |_| Message::ToggleBreakpoint(i)
                );
                
                let line_text = text(line)
                    .size(14)
                    .style(if is_current {
                        theme::Text::Color(HIGHLIGHT_COLOR)
                    } else {
                        theme::Text::Color(TEXT_COLOR)
                    });
                
                let row = row![line_number, breakpoint, line_text]
                    .spacing(8)
                    .align_items(alignment::Alignment::Center);

                let styled_row = if is_current {
                    container(row)
                        .style(theme::Container::Custom(Box::new(ContentContainer)))
                        .padding([4, 8])
                        .width(Length::Fill)
                } else {
                    container(row)
                        .padding([4, 8])
                        .width(Length::Fill)
                };

                rows.push(styled_row.into());
            }
        }
        (rows, current_line_idx)
    }
}

impl Application for EmulatorGui {
    type Message = Message;
    type Theme = theme::Theme;
    type Executor = iced::executor::Default;
    type Flags = Arc<Mutex<VM>>;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let program_text = if let Ok(locked_vm) = flags.lock() {
            locked_vm.get_program().join("\n")
        } else {
            String::new()
        };

        (Self {
            vm: flags,
            program_text,
            output: Vec::new(),
            breakpoints: HashSet::new(),
            memory_start_addr: String::from("0"),
            is_running: false,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("ILOC Emulator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Step => {
                if let Ok(mut vm) = self.vm.lock() {
                    if vm.step() {
                        let (_, _, pc) = vm.get_state();
                        // Get any new output
                        self.output = vm.get_output().to_vec();
                        if self.breakpoints.contains(&pc) {
                            self.is_running = false;
                        }
                        // Scroll to current instruction
                        return Command::perform(
                            std::future::ready(pc as f32 * 30.0),
                            Message::ScrollTo
                        );
                    }
                }
                self.log_state();
                Command::none()
            }
            Message::Run => {
                self.is_running = true;
                if let Ok(mut vm) = self.vm.lock() {
                    while vm.step() {
                        let (_, _, pc) = vm.get_state();
                        if self.breakpoints.contains(&pc) {
                            self.is_running = false;
                            break;
                        }
                    }
                    // Get final output
                    self.output = vm.get_output().to_vec();
                }
                self.log_state();
                Command::none()
            }
            Message::Continue => {
                self.is_running = true;
                if let Ok(mut vm) = self.vm.lock() {
                    while vm.step() {
                        let (_, _, pc) = vm.get_state();
                        if self.breakpoints.contains(&pc) {
                            self.is_running = false;
                            break;
                        }
                    }
                    // Get final output
                    self.output = vm.get_output().to_vec();
                }
                self.log_state();
                Command::none()
            }
            Message::Reset => {
                if let Ok(mut vm) = self.vm.lock() {
                    *vm = VM::new(1024);
                    if let Ok(program) = crate::parser::parse_iloc(&self.program_text) {
                        vm.load_program(program);
                    }
                    vm.clear_output();
                    self.output.clear();
                    self.is_running = false;
                }
                self.log_state();
                Command::none()
            }
            Message::ToggleBreakpoint(line) => {
                if self.breakpoints.contains(&line) {
                    self.breakpoints.remove(&line);
                } else {
                    self.breakpoints.insert(line);
                }
                Command::none()
            }
            Message::UpdateMemoryStart(addr) => {
                self.memory_start_addr = addr;
                Command::none()
            }
            Message::ClearOutput => {
                if let Ok(mut vm) = self.vm.lock() {
                    vm.clear_output();
                }
                self.output.clear();
                Command::none()
            }
            Message::AddOutput(text) => {
                self.output.push(text);
                Command::none()
            }
            Message::ScrollTo(offset) => {
                let cmd: Command<Message> = scrollable::scroll_to(
                    scrollable::Id::new("instructions"),
                    scrollable::AbsoluteOffset { x: 0.0, y: offset }
                );
                cmd
            }
            _ => Command::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let memory_start = self.memory_start_addr.parse().unwrap_or(0);
        let memory_view = scrollable(
            text(self.format_memory_view(memory_start))
                .font(iced::Font::MONOSPACE)
                .size(14)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .style(theme::Scrollable::default());

        let (instruction_rows, current_line_idx) = self.instruction_rows();
        let instructions = scrollable(
            column(instruction_rows).spacing(0)
        )
        .id(scrollable::Id::new("instructions"))
        .height(Length::Fill)
        .width(Length::Fill)
        .style(theme::Scrollable::default());

        let registers_view = container(
            scrollable(
                text(self.format_registers())
                    .font(iced::Font::MONOSPACE)
                    .size(14)
            )
            .height(Length::Fill)
            .style(theme::Scrollable::default())
        )
        .padding(10)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(ContentContainer)));

        let output_view = container(
            scrollable(
                text(self.output.join("\n"))
                    .font(iced::Font::MONOSPACE)
                    .size(14)
            )
            .height(Length::Fill)
            .style(theme::Scrollable::default())
        )
        .padding(10)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(ContentContainer)));

        let content = row![
            // Left panel - Instructions
            column![
                row![
                    button("Step").on_press(Message::Step).style(theme::Button::Secondary),
                    button("Run").on_press(Message::Run).style(theme::Button::Secondary),
                    button("Reset").on_press(Message::Reset).style(theme::Button::Secondary),
                    button("Continue").on_press(Message::Continue).style(theme::Button::Secondary)
                ].spacing(10),
                text("Instructions:").size(16),
                instructions
            ]
            .spacing(10)
            .width(Length::FillPortion(2)),

            // Right panel - Memory, Registers, Output
            column![
                // Memory section
                column![
                    text("Memory:").size(16),
                    memory_view
                ]
                .spacing(2)
                .height(Length::FillPortion(2)),

                // Registers section
                column![
                    text("Registers:").size(16),
                    registers_view
                ]
                .spacing(2)
                .height(Length::FillPortion(1)),

                // Output section
                column![
                    text("Output:").size(16),
                    output_view
                ]
                .spacing(2)
                .height(Length::FillPortion(1))
            ]
            .spacing(10)
            .width(Length::FillPortion(3))
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(theme::Container::Custom(Box::new(CustomContainer)))
            .into()
    }

    fn theme(&self) -> Self::Theme {
        theme::Theme::Light
    }
}

pub fn run_gui(vm: Arc<Mutex<VM>>) -> iced::Result {
    let mut settings = Settings::with_flags(vm);
    settings.window.size = (1600, 1000);  // Increased window size
    EmulatorGui::run(settings)
} 