use crate::assembler::Assembler;
use crate::cpu::CPU;
use macroquad::prelude::*;
use macroquad::ui;
use macroquad::ui::{root_ui, Ui};
use macroquad::ui::widgets::Group;
use std::fs;
use ui::{hash, widgets};
use crate::cpu;

enum CurrentAction {
    SelectProgram(String),
    RunProgram,
    ViewProgram,
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
        CurrentAction::ViewProgram => draw_program_view(state),
        _ => draw_main_window(state),
    };
}

fn describe_program(ui: &mut Ui, program: &Vec<String>) {
    Group::new(hash!(), vec2(screen_width()/2., screen_height()))
        .position(vec2(10., 50.))
        .ui(ui, |ui| {
            for line in program {
                ui.label(None, line);
            }
        });
}

fn draw_program_view(state: &mut AppState) {
    let program = state.assembler.as_ref().unwrap().view_program();
    widgets::Window::new(hash!(), vec2(0., 0.), vec2(screen_width(), screen_height()))
        .label("View Program")
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            if ui.button(vec2(300., 10.), "Back") {
                state.cur_state = CurrentAction::Wait;
            }

            describe_program(ui, &program)
        });
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
                    }

                    if ui.button(vec2(250., 10.), "Reset") {
                        state.cpu.reset();
                    }
                    if ui.button(vec2(300., 10.), "Back") {
                        state.cur_state = CurrentAction::Wait;
                        state.cpu.reset();
                    }

                    describe_mem_reg(ui, &state.cpu);
                });
            describe_cpu(ui, &state.cpu);
        });
}

fn describe_cpu(ui: &mut Ui,cpu: &cpu::CPU)  {
    let info = cpu.view_instr_info();
    if info.name.is_none() {
        return;
    }
    Group::new(hash!(), vec2(screen_width()/2., 100.))
        .position(vec2(screen_width()/2. + 20., 50.))
        .ui(ui, |ui| {
            ui.label(None, &format!("Instruction: {}", info.name.clone().unwrap()));
            if info.rd.is_some() {
                ui.label(None, &format!("RD: {}", info.rd.unwrap()));
            }
            ui.label(None, &format!("R1: {}", info.rs1));
            if info.rs2.is_some() {
                ui.label(None, &format!("R2: {}", info.rs2.unwrap()));
            }
            if info.imm.is_some() {
                ui.label(None, &format!("IMM: {}", info.imm.unwrap()));
            }
        });
}

fn describe_mem_reg(ui: &mut Ui,cpu: &cpu::CPU)  {
    Group::new(hash!(), vec2(screen_width()/2., 3200.))
        .position(vec2(10., 50.))
        .ui(ui, |ui| {
            let mut i: u32 = 0;
            for x in cpu.view_registers() {
                ui.label(None, &format!("x{}: {}", i, *x as i32));
                i += 1;
            }

            let mut j: u32 = 0;
            for x in cpu.view_memory() {
                ui.label(vec2(100., 15.*j as f32), &format!("M[{}]: 0x{:x}", j, *x as i32));
                j += 1;
            }
        });
}

fn draw_main_window(state: &mut AppState) {
    let names = get_file_names();
    widgets::Window::new(hash!(), vec2(0., 0.), vec2(screen_width(), screen_height()))
        .label("Test")
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            Group::new(hash!(), vec2(screen_width() - 20.0, screen_height() - 20.0))
                .position(vec2(10., 10.))
                .ui(ui, |ui| {
                    change_skin(ui);

                    Group::new(hash!(), vec2(screen_width() - 20., 200.))
                        .position(vec2(10., 10.))
                        .ui(ui, |ui| {
                            for n in names {
                                if ui.button(None, n.clone()) {
                                    state.cur_state = CurrentAction::SelectProgram(n.clone());
                                    set_program(state);
                                }
                            }
                        });


                    Group::new(hash!(), vec2(screen_width() - 20., 50.))
                        .position(vec2(screen_width()/2., 10.))
                        .ui(ui, |ui| {
                            if let CurrentAction::SelectProgram(p) = &state.cur_state {
                                if ui.button(None, format!("Run {}", p)) {
                                    state.cur_state = CurrentAction::RunProgram;
                                }
                            }

                            if let CurrentAction::SelectProgram(p) = &state.cur_state {
                                if ui.button(None, format!("View {}", p)) {
                                    state.cur_state = CurrentAction::ViewProgram;
                                }
                            }
                        });
                });
        });
}

fn set_program(state: &mut AppState) {
    if let CurrentAction::SelectProgram(n) = &mut state.cur_state {
        let assembler = Assembler::open_file(&format!("./programs/{}.rv", n));
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

fn change_skin(ui: &mut Ui) {
    let mut st = ui.default_skin();

    st.window_style = ui.style_builder().color(BLACK).text_color(WHITE).build();
    st.label_style = ui.style_builder().text_color(WHITE).build();
    st.button_style = ui
        .style_builder()
        .color(GRAY)
        .color_hovered(DARKGRAY)
        .text_color_hovered(GRAY)
        .text_color(WHITE)
        .margin(RectOffset::new(2.5,2.5,0.5,0.5))
        .build();

    ui.push_skin(&st.clone());
}