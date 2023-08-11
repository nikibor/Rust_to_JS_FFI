use neon::prelude::*;

fn factorial(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as u64;
    let result = (1..=n).product::<u64>();
    Ok(cx.number(result as f64))
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("factorial", factorial)?;
    Ok(())
}
