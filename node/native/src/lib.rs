use jolocom_native_utils::{get_icp_str, get_id_from_event_str, validate_events_str};
use neon::prelude::*;

fn get_icp(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(get_icp_str()))
}

fn validate_events(mut cx: FunctionContext) -> JsResult<JsString> {
    let str = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(validate_events_str(str)))
}

fn get_id_from_event(mut cx: FunctionContext) -> JsResult<JsString> {
    let str = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(get_id_from_event_str(str)))
}

register_module!(mut cx, {
    cx.export_function("getIcp", get_icp)?;
    cx.export_function("validateEvents", validate_events)?;
    cx.export_function("getIdFromEvent", get_id_from_event)?;
    Ok(())
});
