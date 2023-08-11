use neon::macro_internal::runtime::array::len;
use neon::prelude::*;

pub mod rayon_worker;

// pub fn rayon_js_binding(mut cx: FunctionContext) -> JsResult<JsNumber>  {
//     let input_data = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
//     let data = Vec::new();
//
//     // for item in input_data{
//     //     let item_rust = item.downcast::<JsObject>().unwrap();
//     // }
//     let sums = rayon_worker::RayonWorker::sum_of_squares(&input_data);
//     Ok(cx.number(sums as f64))
// }