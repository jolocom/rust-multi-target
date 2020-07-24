use jolocom_native_utils::{get_icp_str, get_id_from_event_str, validate_events_str, wallet};
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

fn new_wallet(mut cx: FunctionContext) -> JsResult<JsString> {
    let id = cx.argument::<JsString>(0)?.value();
    let pass = cx.argument::<JsString>(1)?.value();
    Ok(cx.string(wallet::new_wallet(id, pass)))
}

fn change_pass(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let old_pass = cx.argument::<JsString>(2)?.value();
    let new_pass = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::change_pass(ew, id, old_pass, new_pass)))
}

fn new_key(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let key_type = cx.argument::<JsString>(3)?.value();
    let controller = match cx.argument::<JsString>(4) {
        Ok(s) => Some(vec![s.value()]),
        Err(e) => None,
    };
    Ok(cx.string(wallet::new_key(ew, id, pass, key_type, controller)))
}

fn get_keys(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    Ok(cx.string(wallet::get_keys(ew, id, pass)))
}

fn sign(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let data = cx.argument::<JsString>(3)?.value();
    let key_ref = cx.argument::<JsString>(4)?.value();
    Ok(cx.string(wallet::sign(ew, id, pass, data, key_ref)))
}

fn verify(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let data = cx.argument::<JsString>(3)?.value();
    let key_ref = cx.argument::<JsString>(4)?.value();
    let sig = cx.argument::<JsString>(4)?.value();
    Ok(cx.boolean(wallet::verify(ew, id, pass, data, key_ref, sig)))
}

fn get_random(mut cx: FunctionContext) -> JsResult<JsString> {
    let len = cx.argument::<JsNumber>(0)?.value() as usize;
    Ok(cx.string(wallet::get_random_b64(len.into())))
}

register_module!(mut cx, {
    cx.export_function("getIcp", get_icp)?;
    cx.export_function("validateEvents", validate_events)?;
    cx.export_function("getIdFromEvent", get_id_from_event)?;
    cx.export_function("newWallet", new_wallet)?;
    cx.export_function("changePass", change_pass)?;
    cx.export_function("newKey", new_key)?;
    cx.export_function("getKeys", get_keys)?;
    cx.export_function("sign", sign)?;
    cx.export_function("verify", verify)?;
    cx.export_function("getRandom", get_random)?;
    Ok(())
});
