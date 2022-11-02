use de_cent::{DecentActor, Error};

pub fn create_actor(path: String) -> Result<(), Error> {
    DecentActor::new(path)?;
    Ok(())
}

pub fn create_actor_from_config_str(config: String) {
    let sdk_config = 
    
}
