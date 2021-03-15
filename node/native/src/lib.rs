use jolocom_native_utils::{did_document, keri, wallet};
use neon::prelude::*;
use serde_json::to_string;

fn process_events(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let str = cx.argument::<JsString>(0)?.value();
    let path = cx.argument::<JsString>(1)?.value();
    keri::process_events(&str.as_bytes(), &path).unwrap();
    Ok(JsUndefined::new())
}

fn resolve_id(mut cx: FunctionContext) -> JsResult<JsString> {
    let str = cx.argument::<JsString>(0)?.value();
    let path = cx.argument::<JsString>(1)?.value();
    let method_name = cx.argument::<JsString>(2)?.value();
    Ok(cx.string(
        to_string(&did_document::state_to_did_document(
            keri::get_state(&str.parse().unwrap(), &path)
                .unwrap()
                .unwrap(),
            &method_name,
        ))
        .unwrap(),
    ))
}

fn get_kerl(mut cx: FunctionContext) -> JsResult<JsString> {
    let id_str = cx.argument::<JsString>(0)?.value();
    let path = cx.argument::<JsString>(1)?.value();
    Ok(cx.string(
        String::from_utf8(
            keri::get_kerl(&id_str.parse().unwrap(), &path)
                .unwrap()
                .unwrap(),
        )
        .unwrap(),
    ))
}

fn new_wallet(mut cx: FunctionContext) -> JsResult<JsString> {
    let id = cx.argument::<JsString>(0)?.value();
    let pass = cx.argument::<JsString>(1)?.value();
    Ok(cx.string(wallet::new_wallet(&id, &pass).unwrap()))
}

fn keri_incept_wallet(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    Ok(cx.string(wallet::incept_wallet(&ew, &id, &pass).unwrap()))
}

fn keri_incept_wallet_from_keys(mut cx: FunctionContext) -> JsResult<JsString> {
    let keys_0 = cx.argument::<JsString>(0)?.value();
    let keys_1 = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();

    Ok(cx.string(wallet::incept_populated_wallet(&keys_0, &keys_1, &pass).unwrap()))
}

fn change_pass(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let old_pass = cx.argument::<JsString>(2)?.value();
    let new_pass = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::change_pass(&ew, &id, &old_pass, &new_pass).unwrap()))
}

fn change_id(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let new_id = cx.argument::<JsString>(2)?.value();
    let pass = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::change_id(&ew, &id, &new_id, &pass).unwrap()))
}

fn new_key(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let key_type = cx.argument::<JsString>(3)?.value();

    let controller = match cx.argument_opt(4) {
        Some(optional_controller) => match optional_controller.downcast::<JsString>() {
            Ok(controller) => Some(vec![controller.value()]),
            Err(_) => None,
        },
        None => None,
    };

    Ok(cx.string(wallet::new_key(&ew, &id, &pass, &key_type, controller).unwrap()))
}

fn add_content(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let content = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::add_content(&ew, &id, &pass, &content).unwrap()))
}

fn set_key_controller(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let key_ref = cx.argument::<JsString>(3)?.value();
    let controller = cx.argument::<JsString>(4)?.value();
    Ok(cx.string(wallet::set_key_controller(&ew, &id, &pass, &key_ref, &controller).unwrap()))
}

fn get_key(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let key_ref = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::get_key(&ew, &id, &pass, &key_ref).unwrap()))
}

fn get_key_by_controller(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let controller = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::get_key_by_controller(&ew, &id, &pass, &controller).unwrap()))
}

fn get_keys(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    Ok(cx.string(wallet::get_keys(&ew, &id, &pass).unwrap()))
}

fn sign(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let key_ref = cx.argument::<JsString>(3)?.value();
    let data = cx.argument::<JsString>(4)?.value();
    Ok(cx.string(wallet::sign_by_controller(&ew, &id, &pass, &key_ref, &data).unwrap()))
}

fn verify(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let key = cx.argument::<JsString>(0)?.value();
    let key_type = cx.argument::<JsString>(1)?.value();
    let data = cx.argument::<JsString>(2)?.value();
    let signature = cx.argument::<JsString>(3)?.value();
    Ok(cx.boolean(wallet::verify(&key, &key_type, &data, &signature).unwrap()))
}

fn encrypt(mut cx: FunctionContext) -> JsResult<JsString> {
    let key = cx.argument::<JsString>(0)?.value();
    let key_type = cx.argument::<JsString>(1)?.value();
    let data = cx.argument::<JsString>(2)?.value();

    let aad = match cx.argument_opt(5) {
        Some(aad_arg) => match aad_arg.downcast::<JsString>() {
            Ok(aad_str) => aad_str.value(),
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    };

    Ok(cx.string(wallet::encrypt(&key, &key_type, &data, &aad).unwrap()))
}

fn decrypt(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let controller = cx.argument::<JsString>(3)?.value();
    let data = cx.argument::<JsString>(4)?.value();

    let aad = match cx.argument_opt(5) {
        Some(aad_arg) => match aad_arg.downcast::<JsString>() {
            Ok(aad_str) => aad_str.value(),
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    };

    Ok(
        cx.string(
            wallet::decrypt_by_controller(&ew, &id, &pass, &controller, &data, &aad).unwrap(),
        ),
    )
}

fn ecdh_key_agreement(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let controller = cx.argument::<JsString>(3)?.value();
    let pub_key = cx.argument::<JsString>(4)?.value();

    Ok(cx.string(
        wallet::ecdh_get_shared_secret_by_controller(&ew, &id, &pass, &controller, &pub_key)
            .unwrap(),
    ))
}

fn get_random(mut cx: FunctionContext) -> JsResult<JsString> {
    let len = cx.argument::<JsNumber>(0)?.value() as usize;
    Ok(cx.string(wallet::get_random_b64(len.into()).unwrap()))
}

fn create_didcomm_message(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(wallet::create_didcomm_message()))
}

fn create_jwe_didcomm_message(mut cx: FunctionContext) -> JsResult<JsString> {
    let from = cx.argument::<JsString>(0)?.value();
    let to_handle: Handle<JsArray> = cx.argument(1)?;
    let to_vec: Vec<Handle<JsValue>> = to_handle.to_vec(&mut cx)?;
    let to: Vec<String> = to_vec.iter().fold(Vec::new(), |mut result, elem| {
        result.push(elem.to_string(&mut cx).unwrap().value());
        result
    });
    let to: Vec<&str> = to.iter().map(std::ops::Deref::deref).collect();
    let alg = cx.argument::<JsString>(2)?.value();
    Ok(cx.string(wallet::create_jwe_didcomm_message(&from, &to, &alg)
        .map_err(|e| e.to_string()).unwrap()))
}

fn seal_didcomm_message(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let message = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::seal_didcomm_message(&ew, &id, &pass, &message)
        .map_err(|e| e.to_string()).unwrap()))
}

fn seal_signed_didcomm_message(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let message = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::seal_signed_didcomm_message(
        &ew,
        &id,
        &pass,
        &message,
    ).map_err(|e| e.to_string()).unwrap()))
}

fn receive_didcomm_message(mut cx: FunctionContext) -> JsResult<JsString> {
    let ew = cx.argument::<JsString>(0)?.value();
    let id = cx.argument::<JsString>(1)?.value();
    let pass = cx.argument::<JsString>(2)?.value();
    let message = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(wallet::receive_didcomm_message(
        &ew,
        &id,
        &pass,
        &message.as_bytes(),
    ).map_err(|e| e.to_string()).unwrap()))
}

register_module!(mut cx, {
    cx.export_function("processEvents", process_events)?;
    cx.export_function("resolveId", resolve_id)?;
    cx.export_function("getKerl", get_kerl)?;
    cx.export_function("newWallet", new_wallet)?;
    cx.export_function("keriInceptWalletFromKeys", keri_incept_wallet_from_keys)?;
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
    cx.export_function("ecdhKeyAgreement", ecdh_key_agreement)?;
    cx.export_function("getRandom", get_random)?;
    cx.export_function("createMessage", create_didcomm_message)?;
    cx.export_function("createJweMessage", create_jwe_didcomm_message)?;
    cx.export_function("sealMessage", seal_didcomm_message)?;
    cx.export_function("sealSignedMessage", seal_signed_didcomm_message)?;
    cx.export_function("receiveMessage", receive_didcomm_message)?;
    Ok(())
});
