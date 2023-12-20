use std::{cell::RefCell, rc::Rc, time::Duration};

use egui::{text_edit::CursorRange, Frame, Margin, Sense};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

#[derive(serde::Deserialize, serde::Serialize)]
enum Views {
    Editor,
    Canvas,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    code: String,
    view: Views,
    first_run: bool,
    #[serde(skip)]
    loop_fn: pana_lang::parser::statement::BlockStatement,
    #[serde(skip)]
    environment: pana_lang::eval::environment::RcEnvironment,
    #[serde(skip)]
    evaluator: Option<pana_lang::eval::evaluator::Evaluator>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            code: include_str!("./example.pana").to_string(),
            view: Views::Editor,
            first_run: false,
            loop_fn: Vec::default(),
            environment: Rc::new(RefCell::new(
                pana_lang::eval::environment::Environment::new(None),
            )),
            evaluator: None,
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }

    fn editor(&mut self, ui: &mut egui::Ui) {
        let output = CodeEditor::default()
            .id_source("code editor")
            .with_rows(1)
            .stick_to_bottom(true)
            .vscroll(true)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::GRUVBOX)
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .show(ui, &mut self.code);
        if output.response.has_focus()
            && ui.input(|i| {
                i.modifiers.ctrl
                    && (i.key_pressed(egui::Key::PlusEquals) || i.key_pressed(egui::Key::Minus))
            })
        {
            if let Some(text_cursor_range) = output.cursor_range {
                let text_edit_id = output.response.id;
                self.code.replace_range(
                    text_cursor_range.primary.ccursor.index - 1
                        ..text_cursor_range.primary.ccursor.index,
                    "",
                );
                if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                    state.set_cursor_range(Some(CursorRange::one(
                        output
                            .galley
                            .cursor_left_one_character(&text_cursor_range.primary),
                    )));
                    state.store(ui.ctx(), text_edit_id);
                }
            }
        }
    }

    fn canvas(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ctx.request_repaint_after(Duration::from_millis(16)); //60fps
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::drag());
        let canvas_rect = response.rect;
        if self.first_run {
            let lexer = pana_lang::lexer::Lexer::new(self.code.chars().collect());
            let mut parser = pana_lang::parser::Parser::new(lexer);
            let mut program = parser.parse();

            self.loop_fn.clear();
            self.environment = Rc::new(RefCell::new(
                pana_lang::eval::environment::Environment::new(None),
            ));

            self.evaluator = Some(pana_lang::eval::evaluator::Evaluator::new(
                Some(painter),
                canvas_rect.width(),
                canvas_rect.width(),
                canvas_rect.top(),
            ));

            if let Some(error) = parser.error {
                eprintln!("{}", error);
            }

            let evaluator = self.evaluator.as_mut().unwrap();

            if let Ok(loop_fn) = evaluator.extract_loop_fn(&mut program) {
                self.loop_fn = loop_fn;
            } else {
                eprintln!("No se encontro la funcion `Bucle`");
            }

            if let pana_lang::eval::objects::ResultObj::Copy(
                pana_lang::eval::objects::Object::Error(msg),
            ) = evaluator.eval_program(&program, &self.environment)
            {
                eprintln!("{}", msg);
            }
            self.first_run = false;
        }

        let env = Rc::new(RefCell::new(
            pana_lang::eval::environment::Environment::new(Some(self.environment.clone())),
        ));
        let evaluator = self.evaluator.as_mut().unwrap();
        if let pana_lang::eval::objects::ResultObj::Copy(pana_lang::eval::objects::Object::Error(
            msg,
        )) = evaluator.eval_program(&self.loop_fn, &env)
        {
            eprintln!("{}", msg);
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                match self.view {
                    Views::Editor => {
                        if ui.button("Ejecutar").clicked() {
                            self.view = Views::Canvas;
                            self.first_run = true;
                        }
                    }
                    Views::Canvas => {
                        if ui.button("Codigo").clicked() {
                            self.view = Views::Editor;
                        }
                    }
                }
                let _ = ui.button("Manual");
                ui.add_space(16.0);
            })
        });

        egui::CentralPanel::default()
            .frame(Frame::default().inner_margin(Margin::default()))
            .show(ctx, |ui| match self.view {
                Views::Editor => self.editor(ui),
                Views::Canvas => self.canvas(ui, ctx),
            });
    }
}
