use jolocom_native_utils::{get_id_from_event_str, validate_events_str, wallet};
use neon::prelude::*;

fn validate_events(mut cx: FunctionContext) -> JsResult<JsString> {
    let str = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(validate_events_str(&str)))
}

fn get_id_from_event(mut cx: FunctionContext) -> JsResult<JsString> {
    let str = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(get_id_from_event_str(&str)))
}

fn new_wallet(mut cx: FunctionContext) -> JsResult<JsString> {
    let id = cx.argument::<JsString>(0)?.value();
    let pass = cx.argument::<JsString>(1)?.value();
    Ok(cx.string(wallet::new_wallet(&id, &pass)))
}

fn keri_incept_wallet(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    Ok(cx.string(wallet::incept_wallet(&ew, &id, &pass)))
}

fn change_pass(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let old_pass = cx.argument::<JsString>(2)?.value();
    let new_pass = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::change_pass(&ew, &id, &old_pass, &new_pass)))
}

fn change_id(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let new_id = cx.argument::<JsString>(2)?.value();
    let pass = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::change_id(&ew, &id, &new_id, &pass)))
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
    Ok(cx.string(wallet::new_key(&ew, &id, &pass, &key_type, controller)))
}

fn add_content(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let content = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::add_content(&ew, &id, &pass, &content)))
}

fn set_key_controller(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let key_ref = cx.argument::<JsString>(3)?.value();
    let controller = cx.argument::<JsString>(4)?.value();
    Ok(cx.string(wallet::set_key_controller(
        &ew,
        &id,
        &pass,
        &key_ref,
        &controller,
    )))
}

fn get_key(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let key_ref = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::get_key(&ew, &id, &pass, &key_ref)))
}

fn get_key_by_controller(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let controller = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::get_key_by_controller(&ew, &id, &pass, &controller)))
}

fn get_keys(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    Ok(cx.string(wallet::get_keys(&ew, &id, &pass)))
}

fn sign(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let data = cx.argument::<JsString>(3)?.value();
    let key_ref = cx.argument::<JsString>(4)?.value();
    Ok(cx.string(wallet::sign_by_controller(&ew, &id, &pass, &data, &key_ref)))
}

fn verify(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let key = cx.argument::<JsString>(0)?.value();
    let key_type = cx.argument::<JsString>(1)?.value();
    let data = cx.argument::<JsString>(2)?.value();
    let signature = cx.argument::<JsString>(3)?.value();
    Ok(cx.boolean(wallet::verify(&key, &key_type, &data, &signature)))
}

fn encrypt(mut cx: FunctionContext) -> JsResult<JsString> {
    let key = cx.argument::<JsString>(0)?.value();
    let key_type = cx.argument::<JsString>(1)?.value();
    let data = cx.argument::<JsString>(2)?.value();
    let aad = match cx.argument::<JsString>(3) {
        Ok(s) => s.value(),
        Err(_) => "".to_string(),
    };
    Ok(cx.string(wallet::encrypt(&key, &key_type, &data, &aad)))
}

fn decrypt(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let controller = cx.argument::<JsString>(4)?.value();
    let data = cx.argument::<JsString>(3)?.value();
    let aad = match cx.argument::<JsString>(2) {
        Ok(s) => s.value(),
        Err(_) => "".to_string(),
    };
    Ok(cx.string(wallet::decrypt_by_controller(&ew, &id, &pass, &controller, &data, &aad)))
}

fn get_random(mut cx: FunctionContext) -> JsResult<JsString> {
    let len = cx.argument::<JsNumber>(0)?.value() as usize;
    Ok(cx.string(wallet::get_random_b64(len.into())))
}

register_module!(mut cx, {
    cx.export_function("validateEvents", validate_events)?;
    cx.export_function("getIdFromEvent", get_id_from_event)?;
    cx.export_function("newWallet", new_wallet)?;
    cx.export_function("keriInceptWallet", keri_incept_wallet)?;
    cx.export_function("changePass", change_pass)?;
    cx.export_function("changeId", change_id)?;
    cx.export_function("newKey", new_key)?;
    cx.export_function("addContent", add_content)?;
    cx.export_function("getKey", get_key)?;
    cx.export_function("getKeyByController", get_key_by_controller)?;
    cx.export_function("setKeyController", set_key_controller)?;
    cx.export_function("getKeys", get_keys)?;
    cx.export_function("sign", sign)?;
    cx.export_function("verify", verify)?;
    cx.export_function("encrypt", encrypt)?;
    cx.export_function("decrypt", decrypt)?;
    cx.export_function("getRandom", get_random)?;
    Ok(())
});
