pub mod solvers;
use crate::solvers::{factorial, hello, sum_of_squares_rayon, read_json_configs};
use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("factorial", factorial)?;
    cx.export_function("rayon", sum_of_squares_rayon)?;
    cx.export_function("read_configs", read_json_configs)?;
    Ok(())
}
