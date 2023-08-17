pub mod solvers;
use crate::solvers::{sum_of_squares_rayon, hello, factorial};
use neon::prelude::*;


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("factorial", factorial)?;
    cx.export_function("rayon", sum_of_squares_rayon)?;
    Ok(())
}
