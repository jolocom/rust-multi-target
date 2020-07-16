use jolocom_native_utils::{get_icp_str, validate_events_str};
use neon::prelude::*;

fn get_icp(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(get_icp_str()))
}

fn validate_events(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(validate_events_str(cx.downcast::<JsString>(0)?)))
}

register_module!(mut cx, {
    cx.export_function("getIcp", get_icp)?;
    cx.export_function("validateEvents", validate_events)?;
    Ok(())
});
