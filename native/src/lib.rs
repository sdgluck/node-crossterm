#[macro_use]
extern crate neon;
extern crate crossterm;

use crossterm::{cursor, terminal, ClearType};
use neon::prelude::*;

fn map_js_clear_type(clear_type: u16) -> ClearType {
    match clear_type {
        0 => ClearType::All,
        1 => ClearType::FromCursorDown,
        2 => ClearType::FromCursorUp,
        3 => ClearType::CurrentLine,
        4 => ClearType::UntilNewLine,
        _ => panic!("bad clear_type, expecting 0-4, are you using the crossterm.ClearType enum?"),
    }
}

// ---
// cursor
// ---

fn pos(mut cx: FunctionContext) -> JsResult<JsObject> {
    let (x, y) = cursor().pos();
    let x_num = cx.number(x as f64);
    let y_num = cx.number(y as f64);
    let obj = JsObject::new(&mut cx);
    obj.set(&mut cx, "x", x_num);
    obj.set(&mut cx, "y", y_num);
    Ok(obj)
}

fn goto(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value();
    let y = cx.argument::<JsNumber>(1)?.value();
    cursor().goto(x as u16, y as u16);
    Ok(cx.undefined())
}

fn move_up(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let y = cx.argument::<JsNumber>(0)?.value();
    cursor().move_up(y as u16);
    Ok(cx.undefined())
}

fn move_down(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut c = cursor();
    let y = cx.argument::<JsNumber>(0)?.value();
    c.move_down(y as u16);
    Ok(cx.undefined())
}

fn move_right(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let y = cx.argument::<JsNumber>(0)?.value();
    cursor().move_right(y as u16);
    Ok(cx.undefined())
}

fn move_left(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let y = cx.argument::<JsNumber>(0)?.value();
    cursor().move_left(y as u16);
    Ok(cx.undefined())
}

fn save_position(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cursor().save_position();
    Ok(cx.undefined())
}

fn reset_position(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cursor().reset_position();
    Ok(cx.undefined())
}

fn hide(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cursor().hide();
    Ok(cx.undefined())
}

fn show(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cursor().show();
    Ok(cx.undefined())
}

fn blink(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let blink = cx.argument::<JsBoolean>(0)?.value();
    cursor().blink(blink);
    Ok(cx.undefined())
}

// ---
// terminal
// ---

fn clear(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value();
    terminal().clear(map_js_clear_type(x as u16));
    Ok(cx.undefined())
}

fn terminal_size(mut cx: FunctionContext) -> JsResult<JsObject> {
    let (x, y) = terminal().terminal_size();
    let x_num = cx.number(x as f64);
    let y_num = cx.number(y as f64);
    let obj = JsObject::new(&mut cx);
    obj.set(&mut cx, "width", x_num);
    obj.set(&mut cx, "height", y_num);
    Ok(obj)
}

fn scroll_up(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value();
    terminal().scroll_up(x as i16);
    Ok(cx.undefined())
}

fn scroll_down(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value();
    terminal().scroll_down(x as i16);
    Ok(cx.undefined())
}

fn set_size(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value();
    let y = cx.argument::<JsNumber>(1)?.value();
    terminal().set_size(x as i16, y as i16);
    Ok(cx.undefined())
}

fn write(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let str = cx.argument::<JsString>(0)?.value();
    terminal().write(str);
    Ok(cx.undefined())
}

register_module!(mut m, {
    // ClearType = {
    //   All: 0,
    //   FromCursorDown: 1,
    //   FromCursorUp: 2,
    //   CurrentLine: 3,
    //   UntilNewLine: 4
    // }
    let js_clear_types = JsObject::new(&mut m);
    let n = m.number(0 as f64);
    js_clear_types.set(&mut m, "All", n)?;
    let n = m.number(1 as f64);
    js_clear_types.set(&mut m, "FromCursorDown", n)?;
    let n = m.number(2 as f64);
    js_clear_types.set(&mut m, "FromCursorUp", n)?;
    let n = m.number(3 as f64);
    js_clear_types.set(&mut m, "CurrentLine", n)?;
    let n = m.number(4 as f64);
    js_clear_types.set(&mut m, "UntilNewLine", n)?;
    m.export_value("ClearType", js_clear_types);

    // cursor
    m.export_function("pos", pos);
    m.export_function("goto", goto);
    m.export_function("move_up", move_up);
    m.export_function("move_down", move_down);
    m.export_function("move_right", move_right);
    m.export_function("move_left", move_left);
    m.export_function("save_position", save_position);
    m.export_function("reset_position", reset_position);
    m.export_function("hide", hide);
    m.export_function("show", show);

    // terminal
    m.export_function("clear", clear);
    m.export_function("terminal_size", terminal_size);
    m.export_function("scroll_up", scroll_up);
    m.export_function("scroll_down", scroll_down);
    m.export_function("set_size", set_size);
    m.export_function("write", write);

    Ok(())
});
