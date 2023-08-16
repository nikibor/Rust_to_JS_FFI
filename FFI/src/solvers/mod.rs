use crate::solvers::rayon_worker::RayonWorker;
use neon::prelude::*;

pub mod parser;
pub mod rayon_worker;

pub fn rayon_js_binding(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_array = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
    let mut data: Vec<i32> = Vec::new();

    for (_, item) in js_array.iter().enumerate() {
        let js_number = item.downcast::<JsNumber, _>(&mut cx).unwrap();
        data.push(js_number.value(&mut cx) as i32);
    }
    let result = RayonWorker::sum_of_squares(data);
    Ok(cx.number(result as f64))
}
