use core::solvers::rayon_executor::RayonWorker;
use neon::types::{JsString, JsNumber, JsArray};
use neon::prelude::FunctionContext;
use neon::result::JsResult;
use neon::context::Context;


pub fn sum_of_squares_rayon(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_array = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
    let mut data: Vec<i32> = Vec::new();

    for (_, item) in js_array.iter().enumerate() {
        let js_number = item.downcast::<JsNumber, _>(&mut cx).unwrap();
        data.push(js_number.value(&mut cx) as i32);
    }
    let result = RayonWorker::sum_of_squares(data);
    Ok(cx.number(result as f64))
}

pub fn factorial(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as u64;
    let result = (1..=n).product::<u64>();
    Ok(cx.number(result as f64))
}

pub fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

pub fn read_json_configs(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let dir_path = cx.argument::<JsString>(0)?.value(&mut cx) as String;
    let configs = RayonWorker::read_configs(&dir_path).unwrap();
    Ok(cx.number(configs.len() as f64))
}
