use crate::assembler::Assembler;
use crate::cpu::CPU;
use macroquad::prelude::*;
use macroquad::ui;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Group;
use std::fs;
use ui::{hash, widgets};

enum CurrentAction {
    SelectProgram(String),
    RunProgram,
    Pause,
    End,
    Wait,
}

pub struct AppState {
    cpu: CPU,
    assembler: Option<Assembler>,
    cur_state: CurrentAction,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cpu: CPU::default(),
            assembler: None,
            cur_state: CurrentAction::Wait,
        }
    }
}

fn get_file_names() -> Vec<String> {
    let dir = fs::read_dir("./programs").unwrap();

    dir.map(|x| {
        x.unwrap()
            .file_name()
            .into_string()
            .unwrap()
            .replace(".rv", "")
    })
    .collect::<Vec<String>>()
}

pub fn update_app(state: &mut AppState) {
    match state.cur_state {
        CurrentAction::RunProgram => draw_cpu_view(state),
        _ => draw_main_window(state),
    };
}

fn draw_cpu_view(state: &mut AppState) {
    match state.cur_state {
        CurrentAction::RunProgram => (),
        _ => draw_main_window(state),
    }
    widgets::Window::new(hash!(), vec2(0., 0.), vec2(screen_width(), screen_height()))
        .label("Test")
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            Group::new(hash!(), vec2(screen_width() - 20.0, screen_height() - 20.0))
                .position(vec2(10., 10.))
                .ui(ui, |ui| {
                    ui.label(None, &format!("Program Counter: {}", state.cpu.get_pc()));
                    if ui.button(None, "Step Program") {
                        state.cpu.step();
                        println!("{:?}", state.cpu.view_instr_info())
                    }
                    let mut i: u32 = 0;
                    for x in state.cpu.view_registers() {
                        ui.label(None, &format!("Register {}: {}", i, *x as i32));
                        i += 1;
                    }

                    if ui.button(vec2(300., 10.), "Reset") {
                        state.cur_state = CurrentAction::End;
                        state.cpu.reset();
                    }

                });
        });
}

fn draw_main_window(state: &mut AppState) {
    let names = get_file_names();
    widgets::Window::new(hash!(), vec2(0., 0.), vec2(screen_width(), screen_height()))
        .label("Test")
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            Group::new(hash!(), vec2(screen_width() - 20.0, 100.))
                .position(vec2(10., 10.))
                .ui(ui, |ui| {
                    for n in names {
                        if ui.button(None, n.clone()) {
                            state.cur_state = CurrentAction::SelectProgram(n.clone());
                        }
                    }

                    let label = match &state.cur_state {
                        CurrentAction::SelectProgram(n) => &format!("Run {}", n.clone()),
                        _ => "Select a program",
                    };

                    if ui.button(None, label) {
                        match &state.cur_state {
                            CurrentAction::SelectProgram(_) => {
                                set_program(state);
                                state.cur_state = CurrentAction::RunProgram;
                            }
                            _ => (),
                        }
                    }
                });
        });
}

fn set_program(state: &mut AppState) {
    if let CurrentAction::SelectProgram(n) = &mut state.cur_state {
        let assembler = Assembler::open_file(&format!("./programs/{}.rv",n));
        let instrs = assembler.assemble();
        let mut prgm: Vec<u8> = vec![];
        for instr in instrs {
            prgm.push((instr & 0xFF) as u8);
            prgm.push(((instr & (0xFF << 8)) >> 8) as u8);
            prgm.push(((instr & (0xFF << 16)) >> 16) as u8);
            prgm.push(((instr & (0xFF << 24)) >> 24) as u8);
        }
        state.assembler = Some(assembler);
        state.cpu.load_program(&prgm);
    }
}
