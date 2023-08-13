use neon::prelude::*;
use crate::solvers::rayon_worker::RayonWorker;

pub mod rayon_worker;
pub mod parser;

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


// fn clean_phone_numbers(mut cx: FunctionContext) -> JsResult<JsArray> {
//     // regex to accept numbers
//     let re = Regex::new("[^0-9.]").unwrap();
//     // take the first argument, which must be an array
//     let js_arr_handle = cx.argument::<JsArray>(0)?;
//     // convert a JsArray to a Rust Vec
//     let js_vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
//     let mut phone_list: Vec<String> = Vec::new();
//     for (_, item) in js_vec.iter().enumerate() {
//         let js_string = item.downcast::<JsString, FunctionContext>().unwrap();
//         phone_list.push(js_string.value());
//     }
//     // create result js array
//     let js_array = JsArray::new(&mut cx, phone_list.len() as u32);
//     // iterate over Rust Vec and map
//     // each value in the Vec to the js array
//     for (i, phone) in phone_list.iter().enumerate() {
//         let clean_number = cx.string(re.replace_all(phone, ""));
//         js_array.set(&mut cx, i as u32, clean_number).unwrap();
//     }
//     Ok(js_array)
// }
