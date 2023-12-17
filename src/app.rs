use std::f32::consts::TAU;

use egui::{text_edit::CursorRange, vec2, Color32, Frame, Margin, Sense, Stroke, Vec2};
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            code: "imprimir(\"Hola mundo\");".into(),
            view: Views::Editor,
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

    fn canvas(&mut self, ui: &mut egui::Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());
        let rect = response.rect;
        let c = rect.center();
        let r = rect.width() / 2.0 - 100.0;
        let color = Color32::from_gray(128);
        let stroke = Stroke::new(1.0, color);
        painter.circle_stroke(c, r, stroke);
        painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
        painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
        painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);
        return;
        let lexer = pana_lang::lexer::Lexer::new(self.code.chars().collect());
        let mut parser = pana_lang::parser::Parser::new(lexer);
        let program = parser.parse();
        let mut evaluator = pana_lang::eval::evaluator::Evaluator::new();
        if let Some(error) = parser.error {
            eprintln!("{}", error);
        }
        if let pana_lang::eval::objects::ResultObj::Copy(pana_lang::eval::objects::Object::Error(
            msg,
        )) = evaluator.eval_program(program)
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
                            }
                        }
                        Views::Canvas => {
                            if ui.button("Editar").clicked() {
                                self.view = Views::Editor;
                            }
                        }
                    }
                    ui.add_space(16.0);
                }
        )});

        egui::CentralPanel::default()
            .frame(Frame::default().inner_margin(Margin::default()))
            .show(ctx, |ui| match self.view {
                Views::Editor => self.editor(ui),
                Views::Canvas => self.canvas(ui),
            });
    }
}
