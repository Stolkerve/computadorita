use crate::{
    eval::{
        environment::RcEnvironment,
        evaluator::{create_msg_err, Evaluator},
        objects::{new_rc_object, Object, ResultObj},
    },
    parser::expression::{ExprType, Expression},
};
use crate::{parser::expression::FnParams, types::Numeric};

use super::{missmatch_args, missmatch_type_arg};

pub trait InternalFnPointer:
    Fn(&mut Evaluator, FnParams, &RcEnvironment, usize, usize) -> ResultObj
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + InternalFnPointer>
    where
        Self: 'a;
}

impl<F> InternalFnPointer for F
where
    F: Fn(&mut Evaluator, FnParams, &RcEnvironment, usize, usize) -> ResultObj + Clone,
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

const DEFAULT_COLOR_EXPR: ExprType = ExprType::NumericLiteral(Numeric::Int(0xFFFFFFFF));

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

fn pana_keys_to_egui_keys(pana_key: &str) -> Option<egui::Key> {
    match pana_key.to_lowercase().as_str() {
        "⏷" | "abajo" => Some(egui::Key::ArrowDown),
        "⏴" | "izquierda" => Some(egui::Key::ArrowLeft),
        "⏵" | "derecha" => Some(egui::Key::ArrowRight),
        "⏶" | "arriba" => Some(egui::Key::ArrowUp),
        "esc" => Some(egui::Key::Escape),
        "tab" => Some(egui::Key::Tab),
        "<-" => Some(egui::Key::Backspace),
        "enter" => Some(egui::Key::Enter),
        "espacio" => Some(egui::Key::Space),
        "insertar" => Some(egui::Key::Insert),
        "suprimir" => Some(egui::Key::Delete),
        "inicio" => Some(egui::Key::Home),
        "fin" => Some(egui::Key::End),
        "piepag" => Some(egui::Key::PageUp),
        "finpag" => Some(egui::Key::PageDown),
        "menos" => Some(egui::Key::Minus),
        "+" => Some(egui::Key::PlusEquals),
        "=" => Some(egui::Key::PlusEquals),
        "0" => Some(egui::Key::Num0),
        "1" => Some(egui::Key::Num1),
        "2" => Some(egui::Key::Num2),
        "3" => Some(egui::Key::Num3),
        "4" => Some(egui::Key::Num4),
        "5" => Some(egui::Key::Num5),
        "6" => Some(egui::Key::Num6),
        "7" => Some(egui::Key::Num7),
        "8" => Some(egui::Key::Num8),
        "9" => Some(egui::Key::Num9),
        "a" => Some(egui::Key::A),
        "b" => Some(egui::Key::B),
        "c" => Some(egui::Key::C),
        "d" => Some(egui::Key::D),
        "e" => Some(egui::Key::E),
        "f" => Some(egui::Key::F),
        "g" => Some(egui::Key::G),
        "h" => Some(egui::Key::H),
        "i" => Some(egui::Key::I),
        "j" => Some(egui::Key::J),
        "k" => Some(egui::Key::K),
        "l" => Some(egui::Key::L),
        "m" => Some(egui::Key::M),
        "n" => Some(egui::Key::N),
        "o" => Some(egui::Key::O),
        "p" => Some(egui::Key::P),
        "q" => Some(egui::Key::Q),
        "r" => Some(egui::Key::R),
        "s" => Some(egui::Key::S),
        "t" => Some(egui::Key::T),
        "u" => Some(egui::Key::U),
        "v" => Some(egui::Key::V),
        "w" => Some(egui::Key::W),
        "x" => Some(egui::Key::X),
        "y" => Some(egui::Key::Y),
        "z" => Some(egui::Key::Z),
        "f1" => Some(egui::Key::F1),
        "f2" => Some(egui::Key::F2),
        "f3" => Some(egui::Key::F3),
        "f4" => Some(egui::Key::F4),
        "f5" => Some(egui::Key::F5),
        "f6" => Some(egui::Key::F6),
        "f7" => Some(egui::Key::F7),
        "f8" => Some(egui::Key::F8),
        "f9" => Some(egui::Key::F9),
        "f10" => Some(egui::Key::F10),
        "f11" => Some(egui::Key::F11),
        "f12" => Some(egui::Key::F12),
        _ => None,
    }
}

pub fn abs(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "abs".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).acos()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.abs()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn acos(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "acos".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).acos()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.acos()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn cos(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "cos".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).cos()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.cos()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn acosh(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "acosh".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).acosh()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.acosh()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn asen(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "asen".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).asin()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.asin()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn asenh(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "asenh".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).asinh()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.asinh()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn atan(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "atan".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).atan()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.atan()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn atanh(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "atanh".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).atanh()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.atanh()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn atan2(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(2, args.len(), "atan2".len(), line, col);
    }

    let x_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let y_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let y: f64;
    if let ResultObj::Copy(Object::Numeric(num)) = y_obj {
        match num {
            Numeric::Int(a) => y = a as f64,
            Numeric::Float(a) => y = a,
        };
    } else {
        return missmatch_type_arg("numerico", &y_obj.get_type(), line, col);
    }

    if let ResultObj::Copy(Object::Numeric(num)) = x_obj {
        match num {
            Numeric::Int(a) => {
                ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).atan2(y))))
            }
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.atan2(y)))),
        }
    } else {
        missmatch_type_arg("numerico", &x_obj.get_type(), line, col)
    }
}

pub fn cosh(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "cosh".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).cosh()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.cosh()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn exp(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "exp".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).exp()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.exp()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn log(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(2, args.len(), "log".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let base_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let base: f64;

    if let ResultObj::Copy(Object::Numeric(num)) = base_obj {
        match num {
            Numeric::Int(a) => base = a as f64,
            Numeric::Float(a) => base = a,
        };
    } else {
        return missmatch_type_arg("numerico", &base_obj.get_type(), line, col);
    }

    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => {
                ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).log(base))))
            }
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.log(base)))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn log10(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "log10".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).log10()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.log10()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn piso(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "[iso".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.floor()))),
            n => ResultObj::Copy(Object::Numeric(n)),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn pot(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(2, args.len(), "pot".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let base_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let base: f64;

    if let ResultObj::Copy(Object::Numeric(num)) = base_obj {
        match num {
            Numeric::Int(a) => base = a as f64,
            Numeric::Float(a) => base = a,
        };
    } else {
        return missmatch_type_arg("numerico", &base_obj.get_type(), line, col);
    }

    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => {
                ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).powf(base))))
            }
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.powf(base)))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn max(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(2, args.len(), "max".len(), line, col);
    }

    let a_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let b_obj = eval.eval_expression(args.get(1).unwrap(), env);

    match (a_obj, b_obj) {
        (ResultObj::Copy(Object::Numeric(a_num)), ResultObj::Copy(Object::Numeric(b_num))) => {
            match (a_num, b_num) {
                (Numeric::Int(a), Numeric::Int(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Int(a.max(b))))
                }
                (Numeric::Int(a), Numeric::Float(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Int(a.max(b as i64))))
                }
                (Numeric::Float(a), Numeric::Int(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Float(a.max(b as f64))))
                }
                (Numeric::Float(a), Numeric::Float(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Float(a.max(b))))
                }
            }
        }
        _ => ResultObj::Copy(Object::Error(create_msg_err(
            "Se debe ser un tipo de dato numerico".into(),
            line,
            col,
        ))),
    }
}

// 666
pub fn min(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(2, args.len(), "min".len(), line, col);
    }

    let a_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let b_obj = eval.eval_expression(args.get(1).unwrap(), env);

    match (a_obj, b_obj) {
        (ResultObj::Copy(Object::Numeric(a_num)), ResultObj::Copy(Object::Numeric(b_num))) => {
            match (a_num, b_num) {
                (Numeric::Int(a), Numeric::Int(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Int(a.min(b))))
                }
                (Numeric::Int(a), Numeric::Float(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Int(a.min(b as i64))))
                }
                (Numeric::Float(a), Numeric::Int(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Float(a.min(b as f64))))
                }
                (Numeric::Float(a), Numeric::Float(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Float(a.min(b))))
                }
            }
        }
        _ => ResultObj::Copy(Object::Error(create_msg_err(
            "Se debe ser un tipo de dato numerico".into(),
            line,
            col,
        ))),
    }
}

pub fn raiz(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "raiz".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).sqrt()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.sqrt()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn redondear(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "redondear".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);

    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).acos()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.abs()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn sen(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "sen".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).sin()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.sin()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn senh(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "senh".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).sinh()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.sinh()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn tan(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "tan".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).tan()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.tan()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn tanh(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "tanh".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).tanh()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.tanh()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

pub fn techo(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "techo".len(), line, col);
    }

    let num_obj = eval.eval_expression(args.get(0).unwrap(), env);
    if let ResultObj::Copy(Object::Numeric(num)) = num_obj {
        match num {
            Numeric::Int(a) => ResultObj::Copy(Object::Numeric(Numeric::Float((a as f64).ceil()))),
            Numeric::Float(a) => ResultObj::Copy(Object::Numeric(Numeric::Float(a.ceil()))),
        }
    } else {
        missmatch_type_arg("numerico", &num_obj.get_type(), line, col)
    }
}

// Funcion que retorna la longitud de un string o array
pub fn longitud(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "longitud".len(), line, col);
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

// Funcion que retorna el tipo de dato del objeto
pub fn tipo(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "tipo".len(), line, col);
    }

    let arg_obj = eval.eval_expression(args.get(0).unwrap(), env);
    match arg_obj {
        ResultObj::Copy(obj) => ResultObj::Ref(new_rc_object(Object::String(obj.get_type()))),
        ResultObj::Ref(obj) => {
            ResultObj::Ref(new_rc_object(Object::String(obj.borrow().get_type())))
        }
    }
}

pub fn cadena(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "cadena".len(), line, col);
    }

    let arg_obj = eval.eval_expression(args.get(0).unwrap(), env);
    match arg_obj {
        ResultObj::Copy(obj) => ResultObj::Ref(new_rc_object(Object::String(obj.to_string()))),
        ResultObj::Ref(obj) => {
            ResultObj::Ref(new_rc_object(Object::String(obj.borrow().to_string())))
        }
    }
}

//                      texto, x, y, tamano de fuente
// dibujar_texto("hola mundo", 0, 0, 14);
pub fn dibujar_texto(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() < 4 || args.len() > 5 {
        return missmatch_args(4, args.len(), "dibujar_texto".len(), line, col);
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
            return missmatch_type_arg("dibujar_texto", &obj.get_type(), line, col)
        }
        ResultObj::Ref(obj) => match &*obj.borrow() {
            Object::String(string) => {
                text = string.clone();
            }
            obj => return missmatch_type_arg("cadena", &obj.get_type(), line, col),
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
            return ResultObj::Copy(Object::Error(create_msg_err(
                "Se debe ser un tipo de dato numerico".into(),
                line,
                col,
            )))
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
pub fn dibujar_linea(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() < 4 || args.len() > 5 {
        return missmatch_args(4, args.len(), "dibujar_linea".len(), line, col);
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
            return ResultObj::Copy(Object::Error(create_msg_err(
                "Se espera un tipo de dato numerico".into(),
                line,
                col,
            )))
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

// dibujar_rectangulo(posX, posY, scaleX, scaleY)
pub fn dibujar_rectangulo(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() < 4 || args.len() > 5 {
        return missmatch_args(4, args.len(), "dibujar_rectangulo".len(), line, col);
    }

    let pos_x_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let pos_y_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let scale_x_obj = eval.eval_expression(args.get(2).unwrap(), env);
    let scale_y_obj = eval.eval_expression(args.get(3).unwrap(), env);
    // let filled_obj = eval.eval_expression(args.get(4).unwrap(), env);
    let color_obj = eval.eval_expression(
        args.get(4)
            .unwrap_or(&Expression::new(DEFAULT_COLOR_EXPR, 0, 0)),
        env,
    );

    let pos_x: f32;
    let pos_y: f32;
    let scale_x: f32;
    let scale_y: f32;
    let color: u32;

    match (pos_x_obj, pos_y_obj, scale_x_obj, scale_y_obj, color_obj) {
        (
            ResultObj::Copy(Object::Numeric(pos1_x_num)),
            ResultObj::Copy(Object::Numeric(pos1_y_num)),
            ResultObj::Copy(Object::Numeric(pos2_x_num)),
            ResultObj::Copy(Object::Numeric(pos2_y_num)),
            ResultObj::Copy(Object::Numeric(color_num)),
        ) => {
            pos_x = extract_f32_from_numeric(pos1_x_num);
            pos_y = extract_f32_from_numeric(pos1_y_num);
            scale_x = extract_f32_from_numeric(pos2_x_num);
            scale_y = extract_f32_from_numeric(pos2_y_num);
            color = set_alpha_on_u32(extract_u32_from_numeric(color_num));
        }
        _ => {
            return ResultObj::Copy(Object::Error(create_msg_err(
                "Se espera un tipo de dato numerico".into(),
                line,
                col,
            )))
        }
    }

    let rgba = extract_rgba(color);
    if let Some(painter) = eval.painter.as_mut() {
        painter.rect_filled(
            egui::Rect::from_two_pos(
                egui::Pos2::new(pos_x, pos_y + eval.canvas.top),
                egui::Pos2::new(pos_x + scale_x, pos_y + scale_y + eval.canvas.top),
            ),
            egui::Rounding::ZERO,
            egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3),
        );
    }
    ResultObj::Copy(Object::Void)
}

// dibujar_circulo(0, 0, 40, 0xff0000)
pub fn dibujar_circulo(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() < 3 || args.len() > 4 {
        return missmatch_args(3, args.len(), "dibujar_circulo".len(), line, col);
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
            return ResultObj::Copy(Object::Error(create_msg_err(
                "Se espera un tipo de dato numerico".into(),
                line,
                col,
            )))
        }
    }

    let rgba = extract_rgba(color);
    if let Some(painter) = eval.painter.as_mut() {
        painter.circle_filled(
            egui::Pos2::new(pos_x, pos_y + eval.canvas.top),
            radius,
            egui::Color32::from_rgba_unmultiplied(rgba.0, rgba.1, rgba.2, rgba.3),
        );
    }
    ResultObj::Copy(Object::Void)
}

pub fn lienzo_ancho(
    eval: &mut Evaluator,
    args: FnParams,
    _env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "lienzo_ancho".len(), line, col);
    }

    ResultObj::Copy(Object::Numeric(Numeric::Float(eval.canvas.width as f64)))
}

pub fn lienzo_altura(
    eval: &mut Evaluator,
    args: FnParams,
    _env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "lienzo_altura".len(), line, col);
    }

    ResultObj::Copy(Object::Numeric(Numeric::Float(eval.canvas.height as f64)))
}

pub fn tecla_presionada(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    line: usize,
    col: usize,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(1, args.len(), "tecla_presionada".len(), line, col);
    }
    let key_obj = eval.eval_expression(args.get(0).unwrap(), env);
    match key_obj {
        ResultObj::Copy(obj) => missmatch_type_arg("cadena", &obj.to_string(), line, col),
        ResultObj::Ref(obj) => match &*obj.borrow() {
            Object::String(pana_key) => {
                if let Some(inputs) = &eval.inputs {
                    if let Some(egui_key) = pana_keys_to_egui_keys(pana_key) {
                        return ResultObj::Copy(Object::Boolean(inputs.contains(&egui_key)));
                    }
                }
                ResultObj::Copy(Object::Boolean(false))
            }
            obj => missmatch_type_arg("cadena", &obj.to_string(), line, col),
        },
    }
}
