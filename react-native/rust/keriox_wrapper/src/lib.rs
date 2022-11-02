mod export;
use jolocom_native_utils::{create_actor, create_actor_from_config_str};

export! {
    @Java_io_jolocom_jolocomCore_JolocomCoreModule_createIdentity
    fn create_identity(path: String) -> Result<String, String> {
        create_actor(path).map_err(|e| e.to_string())?;
        Ok("done".into())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_createIdentityWithConfig
    fn create_identity_with_config(config: String) -> Result<String, String> {
        create_actor_from_config_str(config).map_err(|e| e.to_string())?;
        Ok("done".into())
    }
}

ffi_support::define_string_destructor!(jolo_destroy_string);
