use crate::eval::{
    evaluator::create_msg_err,
    objects::{Object, ResultObj},
};

pub mod internal;
pub mod member;

fn missmatch_type_arg(
    name: &str,
    obj_type: &str,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    ResultObj::Copy(Object::Error(create_msg_err(
        format!("Se espera un tipo de dato {}, no {}.", name, obj_type),
        target_line,
        target_col + name.len(),
    )))
}

fn missmatch_args(
    max: usize,
    len: usize,
    name_len: usize,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    ResultObj::Copy(Object::Error(create_msg_err(
        format!("Se encontro {} argumentos de {}", len, max),
        target_line,
        target_col + name_len + 3 + len,
    )))
}

fn missmatch_type(name: &str, obj_type: &str, target_line: usize, target_col: usize) -> ResultObj {
    ResultObj::Copy(Object::Error(create_msg_err(
        format!(
            "El tipo de dato {} no posee el miembro `{}`",
            obj_type, name
        ),
        target_line,
        target_col + 2,
    )))
}
