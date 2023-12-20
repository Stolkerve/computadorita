use crate::eval::{
    environment::RcEnvironment,
    evaluator::Evaluator,
    objects::{new_rc_object, Object, ResultObj},
};
use crate::{parser::expression::FnParams, types::Numeric};

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

//                      texto, x, y, tamano de fuente
// dibujar_texto("hola mundo", 0, 0, 14);
pub fn dibujar_texto(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 4 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 4",
            args.len()
        )));
    }
    let text_obj = eval.eval_expression(args.get(0).unwrap(), env);
    let pos_x_obj = eval.eval_expression(args.get(1).unwrap(), env);
    let pos_y_obj = eval.eval_expression(args.get(2).unwrap(), env);
    let font_size_obj = eval.eval_expression(args.get(3).unwrap(), env);

    let text: String;
    let pos_x: f32;
    let pos_y: f32;
    let font_size: f32;

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

    match (pos_x_obj, pos_y_obj, font_size_obj) {
        (
            ResultObj::Copy(Object::Numeric(pos_x_num)),
            ResultObj::Copy(Object::Numeric(pos_y_num)),
            ResultObj::Copy(Object::Numeric(font_size_num)),
        ) => {
            pos_x = extract_f32_from_numeric(pos_x_num);
            pos_y = extract_f32_from_numeric(pos_y_num);
            font_size = extract_f32_from_numeric(font_size_num);
        }
        _ => {
            return ResultObj::Copy(Object::Error(
                format!("Se espera un tipo de dato numerico",),
            ))
        }
    };

    // dibujar
    if let Some(painter) = eval.painter.as_mut() {
        let galley = painter.layout(
            text,
            egui::FontId::monospace(font_size),
            egui::Color32::WHITE,
            eval.canvas.width,
        );
        painter.galley(egui::Pos2::new(pos_x, pos_y + eval.canvas.top), galley);
    }

    ResultObj::Copy(Object::Void)
}
