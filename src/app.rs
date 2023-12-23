use std::{cell::RefCell, rc::Rc, time::Duration};

use egui::{text_edit::CursorRange, Color32, FontId, Frame, Margin, RichText, Sense, Vec2, Vec2b};
use egui_code_editor::{CodeEditor, ColorTheme};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::pana_syntax::pana_syntax;

const MANUAL_STR: &str = include_str!("manual.md");

#[derive(serde::Deserialize, serde::Serialize)]
enum Views {
    Editor,
    Canvas,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    #[serde(skip)]
    code: String,
    #[serde(skip)]
    view: Views,
    #[serde(skip)]
    first_run: bool,
    #[serde(skip)]
    show_manual: bool,
    #[serde(skip)]
    loop_fn: pana_lang::parser::statement::BlockStatement,
    #[serde(skip)]
    environment: pana_lang::eval::environment::RcEnvironment,
    #[serde(skip)]
    evaluator: Option<pana_lang::eval::evaluator::Evaluator>,
    #[serde(skip)]
    err_msg: String,
    #[serde(skip)]
    manual_commonmark_cache: CommonMarkCache,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_manual: false,
            code: include_str!("./example.pana").to_string(),
            view: Views::Editor,
            first_run: false,
            loop_fn: Vec::default(),
            environment: Rc::new(RefCell::new(
                pana_lang::eval::environment::Environment::new(None),
            )),
            evaluator: None,
            err_msg: String::new(),
            manual_commonmark_cache: CommonMarkCache::default(),
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
            .with_syntax(pana_syntax())
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
        if !self.err_msg.is_empty() {
            ui.label(
                RichText::new(&self.err_msg)
                    .color(Color32::RED)
                    .font(FontId::proportional(20.0)),
            );
            return;
        }
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
                self.err_msg = error.to_string();
                return;
            }

            let evaluator = self.evaluator.as_mut().unwrap();

            if let Ok(loop_fn) = evaluator.extract_loop_fn(&mut program) {
                self.loop_fn = loop_fn;
            } else {
                self.err_msg = "No se encontro la funcion `Bucle`".to_string();
                return;
            }

            if let pana_lang::eval::objects::ResultObj::Copy(
                pana_lang::eval::objects::Object::Error(msg),
            ) = evaluator.eval_program(&program, &self.environment)
            {
                self.err_msg = msg;
                return;
            }
            self.first_run = false;
        }

        let env = Rc::new(RefCell::new(
            pana_lang::eval::environment::Environment::new(Some(self.environment.clone())),
        ));

        let evaluator = self.evaluator.as_mut().unwrap();
        evaluator.canvas.width = canvas_rect.width();
        evaluator.canvas.height = canvas_rect.height();
        evaluator.canvas.top = canvas_rect.top();

        if let pana_lang::eval::objects::ResultObj::Copy(pana_lang::eval::objects::Object::Error(
            msg,
        )) = evaluator.eval_program(&self.loop_fn, &env)
        {
            self.err_msg = msg;
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
        if self.show_manual {
            egui::Window::new("Manual")
                .open(&mut self.show_manual)
                .scroll2(Vec2b::new(true, true))
                .max_size(Vec2::new(600.0, 500.0))
                .default_width(600.0)
                .show(ctx, |ui| {
                    let markdown = MANUAL_STR;
                    CommonMarkViewer::new("viewer").show(
                        ui,
                        &mut self.manual_commonmark_cache,
                        markdown,
                    );
                });
        }

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
                            self.err_msg.clear();
                        }
                    }
                }
                if ui.button("Manual").clicked() {
                    self.show_manual = true;
                }
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
