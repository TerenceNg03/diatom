use std::io::Write;

use crate::{DiatomObject, DiatomValue, Interpreter};

pub fn impl_prelude<Buffer: Write>(interpreter: &mut Interpreter<Buffer>) {
    interpreter.add_extern_function("print".to_string(), |state, parameters, out| {
        let mut flag = true;
        for parameter in parameters {
            if flag {
                flag = false
            } else {
                write!(out, ", ").map_err(|err| format!("IoError: {err}"))?;
            }
            match parameter {
                DiatomValue::Unit => write!(out, "()"),
                DiatomValue::Bool(b) => write!(out, "{b}"),
                DiatomValue::Int(i) => write!(out, "{i}"),
                DiatomValue::Float(f) => write!(out, "{f}"),
                DiatomValue::Str(sid) => write!(out, "{}", state.get_string_by_id(*sid).unwrap()),
                DiatomValue::Ref(r) => match state.get_obj_by_ref(*r).unwrap() {
                    DiatomObject::Closure {
                        func_id,
                        parameters: _,
                        reg_size: _,
                        captured: _,
                    } => {
                        writeln!(out, "Closure[{func_id}]")
                    }
                    DiatomObject::NativeFunction(f) => {
                        write!(out, "External function@{:p}", f.as_ptr())
                    }
                    DiatomObject::Table(t) => {
                        write!(out, "Table@{:p}", &t)
                    }
                },
            }
            .map_err(|err| format!("IoError: {err}"))?;
        }
        writeln!(out).map_err(|err| format!("IoError: {err}"))?;
        Ok(DiatomValue::Unit)
    });

    interpreter.add_extern_function("assert".to_string(), |_state, parameters, _out| {
        if parameters.len() != 1 {
            return Err(format!(
                "Assert expected 1 parameter while {} is provided",
                parameters.len()
            ));
        }
        match parameters[0] {
            DiatomValue::Bool(b) => {
                if b {
                    Ok(DiatomValue::Unit)
                } else {
                    Err("Assert failed".to_string())
                }
            }
            _ => Err("Assert on an invalid type that is not bool".to_string()),
        }
    });

    interpreter.add_extern_function("panic".to_string(), |state, parameters, _out| {
        if parameters.len() > 1 {
            return Err(format!(
                "Assert expected 1 or 0 parameter while {} is provided",
                parameters.len()
            ));
        }
        if parameters.is_empty() {
            return Err("Panic triggered here".to_string());
        }
        match parameters[0] {
            DiatomValue::Str(sid) => {
                let reason = state.get_string_by_id(sid).unwrap().to_string();
                Err(format!("Panic triggered: `{reason}`"))
            }
            _ => Err(
                "Panic triggered with invalid type(Can not show non-string parameter)".to_string(),
            ),
        }
    });
}