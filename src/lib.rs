use neon::prelude::*;

//mod demo;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;

    //cx.export_class::<demo::SomeClass>("SomeClass")?;
    Ok(())
}
