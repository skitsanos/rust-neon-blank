use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello from Rust"))
}

fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(a + b))
}

fn greet(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?.value(&mut cx);
    Ok(cx.string(format!("hello, {name}")))
}

fn sum_array(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_array = cx.argument::<JsArray>(0)?;
    let len = js_array.len(&mut cx);
    let mut total = 0.0;
    for i in 0..len {
        let value = js_array.get::<JsNumber, _, _>(&mut cx, i)?.value(&mut cx);
        total += value;
    }
    Ok(cx.number(total))
}

fn stats(mut cx: FunctionContext) -> JsResult<JsObject> {
    let input = cx.argument::<JsArray>(0)?;
    let len = input.len(&mut cx);
    if len == 0 {
        let obj = JsObject::new(&mut cx);
        let count = cx.number(0.0);
        let sum = cx.number(0.0);
        let min = cx.undefined();
        let max = cx.undefined();
        obj.set(&mut cx, "count", count)?;
        obj.set(&mut cx, "sum", sum)?;
        obj.set(&mut cx, "min", min)?;
        obj.set(&mut cx, "max", max)?;
        return Ok(obj);
    }

    let mut sum = 0.0;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for i in 0..len {
        let value = input.get::<JsNumber, _, _>(&mut cx, i)?.value(&mut cx);
        sum += value;
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }

    let obj = JsObject::new(&mut cx);
    let count = cx.number(len as f64);
    let sum_value = cx.number(sum);
    let min_value = cx.number(min);
    let max_value = cx.number(max);
    obj.set(&mut cx, "count", count)?;
    obj.set(&mut cx, "sum", sum_value)?;
    obj.set(&mut cx, "min", min_value)?;
    obj.set(&mut cx, "max", max_value)?;
    Ok(obj)
}

fn parse_json(mut cx: FunctionContext) -> JsResult<JsValue> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    let json: Handle<JsObject> = cx.global("JSON")?;
    let parse = json
        .get::<JsFunction, _, _>(&mut cx, "parse")?;
    let args = vec![cx.string(input).upcast()];
    let value = parse.call(&mut cx, json, args)?;
    Ok(value.upcast())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("add", add)?;
    cx.export_function("greet", greet)?;
    cx.export_function("sumArray", sum_array)?;
    cx.export_function("stats", stats)?;
    cx.export_function("parseJson", parse_json)?;
    Ok(())
}
