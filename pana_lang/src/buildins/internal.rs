use std::time::UNIX_EPOCH;

use crate::{
    eval::{
        environment::RcEnvironment,
        evaluator::Evaluator,
        objects::{new_rc_object, Object, ResultObj},
    },
    parser::expression::{ExprType, Expression},
};
use crate::{parser::expression::FnParams, types::Numeric};

use super::rng::Rng;

pub trait InternalFnPointer: Fn(&mut Evaluator, FnParams, &RcEnvironment) -> ResultObj {
    fn clone_box<'a>(&self) -> Box<dyn 'a + InternalFnPointer>
    where
        Self: 'a;
}

impl<F> InternalFnPointer for F
where
    F: Fn(&mut Evaluator, FnParams, &RcEnvironment) -> ResultObj + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + InternalFnPointer>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + InternalFnPointer> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

// Funcion que retorna la longitud de un string o array
pub fn longitud(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap(), env);
    match arg_obj {
        ResultObj::Copy(obj) => ResultObj::Copy(Object::Error(format!(
            "Se espera un tipo de dato cadena, no {}",
            obj.get_type()
        ))),
        ResultObj::Ref(obj) => match &*obj.borrow() {
            Object::List(objs) => ResultObj::Copy(Object::Numeric(Numeric::Int(objs.len() as i64))),
            Object::Dictionary(pairs) => {
                ResultObj::Copy(Object::Numeric(Numeric::Int(pairs.len() as i64)))
            }
            Object::String(string) => {
                ResultObj::Copy(Object::Numeric(Numeric::Int(string.len() as i64)))
            }
            obj => ResultObj::Copy(Object::Error(format!(
                "Se espera un tipo de dato cadena, no {}",
                obj.get_type()
            ))),
        },
    }
}

// Funcion que imprime en una linea objetos en pantalla
// pub fn imprimir(_eval: &mut Evaluator, _args: FnParams, _env: &RcEnvironment) -> ResultObj {
// if !args.is_empty() {
// let objs = args
//     .iter()
//     .map(|arg| eval.eval_expression(arg, env))
//     .collect::<Vec<_>>();
// let string = objs
//     .iter()
//     .map(|obj| obj.to_string())
//     .collect::<Vec<_>>()
//     .join("");
// }
//     ResultObj::Copy(Object::Void)
// }

// Funcion que retorna el tipo de dato del objeto
pub fn tipo(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap(), env);
    match arg_obj {
        ResultObj::Copy(obj) => ResultObj::Ref(new_rc_object(Object::String(obj.get_type()))),
        ResultObj::Ref(obj) => {
            ResultObj::Ref(new_rc_object(Object::String(obj.borrow().get_type())))
        }
    }
}

pub fn cadena(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap(), env);
    match arg_obj {
        ResultObj::Copy(obj) => ResultObj::Ref(new_rc_object(Object::String(obj.to_string()))),
        ResultObj::Ref(obj) => {
            ResultObj::Ref(new_rc_object(Object::String(obj.borrow().to_string())))
        }
    }
}

fn extract_f32_from_numeric(num: Numeric) -> f32 {
    match num {
        Numeric::Int(v) => v as f32,
        Numeric::Float(v) => v as f32,
    }
}

fn extract_u32_from_numeric(num: Numeric) -> u32 {
    match num {
        Numeric::Int(v) => v as u32,
        Numeric::Float(v) => v as u32,
    }
}

fn set_alpha_on_u32(n: u32) -> u32 {
    if n > 0xFFFFFF {
        return n;
    }
    ((n << 8) & 0xFFFFFF00) | 0x000000FF
}

fn extract_rgba(n: u32) -> (u8, u8, u8, u8) {
    ((n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, (n) as u8)
}

const DEFAULT_COLOR_EXPR: ExprType = ExprType::NumericLiteral(Numeric::Int(0xFFFFFFFF));

//                      texto, x, y, tamano de fuente
// dibujar_texto("hola mundo", 0, 0, 14);
pub fn dibujar_texto(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() < 4 || args.len() > 5 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 4 o 5",
            args.len()
        )));
    }
    let text_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let pos_x_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let pos_y_obj = eval.eval_expression(args.get(2).unwrap(), env);
    let font_size_obj = eval.eval_expression(args.get(3).unwrap(), env);
    let color_obj = eval.eval_expression(
        args.get(4)
            .unwrap_or(&Expression::new(DEFAULT_COLOR_EXPR, 0, 0)),
        env,
    );

    let text: String;
    let pos_x: f32;
    let pos_y: f32;
    let font_size: f32;
    let color: u32;

    match text_obj {
        ResultObj::Copy(obj) => {
            return ResultObj::Copy(Object::Error(format!(
                "Se espera un tipo de dato cadena, no {}",
                obj.get_type()
            )))
        }
        ResultObj::Ref(obj) => match &*obj.borrow() {
            Object::String(string) => {
                text = string.clone();
            }
            obj => {
                return ResultObj::Copy(Object::Error(format!(
                    "Se espera un tipo de dato cadena, no {}",
                    obj.get_type()
                )))
            }
        },
    };

    match (pos_x_obj, pos_y_obj, font_size_obj, color_obj) {
        (
            ResultObj::Copy(Object::Numeric(pos_x_num)),
            ResultObj::Copy(Object::Numeric(pos_y_num)),
            ResultObj::Copy(Object::Numeric(font_size_num)),
            ResultObj::Copy(Object::Numeric(color_num)),
        ) => {
            pos_x = extract_f32_from_numeric(pos_x_num);
            pos_y = extract_f32_from_numeric(pos_y_num);
            font_size = extract_f32_from_numeric(font_size_num);
            color = set_alpha_on_u32(extract_u32_from_numeric(color_num));
        }
        _ => {
            return ResultObj::Copy(Object::Error(
                "Se espera un tipo de dato numerico".to_string(),
            ))
        }
    };

    let rgba = extract_rgba(color);
    if let Some(painter) = eval.painter.as_mut() {
        let galley = painter.layout(
            text,
            egui::FontId::monospace(font_size),
            egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3),
            eval.canvas.width,
        );
        painter.galley(egui::Pos2::new(pos_x, pos_y + eval.canvas.top), galley);
    }

    ResultObj::Copy(Object::Void)
}

// dibujar_linea(0, 0, 50, 50);
pub fn dibujar_linea(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() < 4 || args.len() > 5 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 4 o 5",
            args.len()
        )));
    }

    let pos1_x_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let pos1_y_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let pos2_x_obj = eval.eval_expression(args.get(2).unwrap(), env);
    let pos2_y_obj = eval.eval_expression(args.get(3).unwrap(), env);
    let color_obj = eval.eval_expression(
        args.get(4)
            .unwrap_or(&Expression::new(DEFAULT_COLOR_EXPR, 0, 0)),
        env,
    );

    let pos1_x: f32;
    let pos1_y: f32;
    let pos2_x: f32;
    let pos2_y: f32;
    let color: u32;

    match (pos1_x_obj, pos1_y_obj, pos2_x_obj, pos2_y_obj, color_obj) {
        (
            ResultObj::Copy(Object::Numeric(pos1_x_num)),
            ResultObj::Copy(Object::Numeric(pos1_y_num)),
            ResultObj::Copy(Object::Numeric(pos2_x_num)),
            ResultObj::Copy(Object::Numeric(pos2_y_num)),
            ResultObj::Copy(Object::Numeric(color_num)),
        ) => {
            pos1_x = extract_f32_from_numeric(pos1_x_num);
            pos1_y = extract_f32_from_numeric(pos1_y_num);
            pos2_x = extract_f32_from_numeric(pos2_x_num);
            pos2_y = extract_f32_from_numeric(pos2_y_num);
            color = set_alpha_on_u32(extract_u32_from_numeric(color_num));
        }
        _ => {
            return ResultObj::Copy(Object::Error(
                "Se espera un tipo de dato numerico".to_string(),
            ))
        }
    }

    let rgba = extract_rgba(color);
    if let Some(painter) = eval.painter.as_mut() {
        painter.line_segment(
            [
                egui::Pos2::new(pos1_x, pos1_y + eval.canvas.top),
                egui::Pos2::new(pos2_x, pos2_y + eval.canvas.top),
            ],
            egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3),
            ),
        )
    }
    ResultObj::Copy(Object::Void)
}

// dibujar_rectangulo(0, 0, 100, 100)
pub fn dibujar_rectangulo(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() < 4 || args.len() > 5 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 4 o 5",
            args.len()
        )));
    }

    let pos1_x_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let pos1_y_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let pos2_x_obj = eval.eval_expression(args.get(2).unwrap(), env);
    let pos2_y_obj = eval.eval_expression(args.get(3).unwrap(), env);
    // let filled_obj = eval.eval_expression(args.get(4).unwrap(), env);
    let color_obj = eval.eval_expression(
        args.get(4)
            .unwrap_or(&Expression::new(DEFAULT_COLOR_EXPR, 0, 0)),
        env,
    );

    let pos1_x: f32;
    let pos1_y: f32;
    let pos2_x: f32;
    let pos2_y: f32;
    let color: u32;

    match (pos1_x_obj, pos1_y_obj, pos2_x_obj, pos2_y_obj, color_obj) {
        (
            ResultObj::Copy(Object::Numeric(pos1_x_num)),
            ResultObj::Copy(Object::Numeric(pos1_y_num)),
            ResultObj::Copy(Object::Numeric(pos2_x_num)),
            ResultObj::Copy(Object::Numeric(pos2_y_num)),
            ResultObj::Copy(Object::Numeric(color_num)),
        ) => {
            pos1_x = extract_f32_from_numeric(pos1_x_num);
            pos1_y = extract_f32_from_numeric(pos1_y_num);
            pos2_x = extract_f32_from_numeric(pos2_x_num);
            pos2_y = extract_f32_from_numeric(pos2_y_num);
            color = set_alpha_on_u32(extract_u32_from_numeric(color_num));
        }
        _ => {
            return ResultObj::Copy(Object::Error(
                "Se espera un tipo de dato numerico".to_string(),
            ))
        }
    }

    let rgba = extract_rgba(color);
    if let Some(painter) = eval.painter.as_mut() {
        painter.rect_filled(
            egui::Rect::from_two_pos(
                egui::Pos2::new(pos1_x, pos1_y + eval.canvas.top),
                egui::Pos2::new(pos2_x, pos2_y + eval.canvas.top),
            ),
            egui::Rounding::ZERO,
            egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3),
        );
    }
    ResultObj::Copy(Object::Void)
}

// dibujar_circulo(0, 0, 40, 0xff0000)
pub fn dibujar_circulo(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() < 3 || args.len() > 4 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 3 o 4",
            args.len()
        )));
    }

    let pos_x_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let pos_y_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let radius_obj = eval.eval_expression(args.get(2).unwrap(), env);
    let color_obj = eval.eval_expression(
        args.get(3)
            .unwrap_or(&Expression::new(DEFAULT_COLOR_EXPR, 0, 0)),
        env,
    );

    let pos_x: f32;
    let pos_y: f32;
    let radius: f32;
    let color: u32;

    match (pos_x_obj, pos_y_obj, radius_obj, color_obj) {
        (
            ResultObj::Copy(Object::Numeric(pos_x_num)),
            ResultObj::Copy(Object::Numeric(pos_y_num)),
            ResultObj::Copy(Object::Numeric(radius_num)),
            ResultObj::Copy(Object::Numeric(color_num)),
        ) => {
            pos_x = extract_f32_from_numeric(pos_x_num);
            pos_y = extract_f32_from_numeric(pos_y_num);
            radius = extract_f32_from_numeric(radius_num);
            color = set_alpha_on_u32(extract_u32_from_numeric(color_num));
        }
        _ => {
            return ResultObj::Copy(Object::Error(
                "Se espera un tipo de dato numerico".to_string(),
            ))
        }
    }

    let rgba = extract_rgba(color);
    if let Some(painter) = eval.painter.as_mut() {
        painter.circle_filled(
            egui::Pos2::new(pos_x, pos_y),
            radius,
            egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3),
        );
    }
    ResultObj::Copy(Object::Void)
}

// aleatorio(0, 100) -> [0, 100]
pub fn aleatorio(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 2 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 2",
            args.len()
        )));
    }
    let min_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let max_obj = eval.eval_expression(args.get(1).unwrap(), env);

    match (min_obj, max_obj) {
        (ResultObj::Copy(Object::Numeric(min_num)), ResultObj::Copy(Object::Numeric(max_num))) => {
            let mut rng = Rng::new(
                std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            match (min_num, max_num) {
                (Numeric::Int(a), Numeric::Int(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Int(rng.rand_range_i64(a, b))))
                }
                (Numeric::Int(a), Numeric::Float(b)) => ResultObj::Copy(Object::Numeric(
                    Numeric::Int(rng.rand_range_i64(a, b as i64)),
                )),
                (Numeric::Float(a), Numeric::Int(b)) => ResultObj::Copy(Object::Numeric(
                    Numeric::Int(rng.rand_range_i64(a as i64, b)),
                )),
                (Numeric::Float(a), Numeric::Float(b)) => ResultObj::Copy(Object::Numeric(
                    Numeric::Int(rng.rand_range_i64(a as i64, b as i64)),
                )),
            }
        }
        _ => {
            ResultObj::Copy(Object::Error(
                "Se espera un tipo de dato numerico".to_string(),
            ))
        }
    }
}
