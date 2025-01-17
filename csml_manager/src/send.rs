use crate::{
    data::{DEBUG, DISABLE_SSL_VERIFY},
    ConversationInfo,
};
use curl::{easy::Easy, Error};
use std::env;
use std::io::Read;
use std::time::SystemTime;

fn format_and_transfer(curl: &mut Easy, mut msg: &[u8], result: &mut Vec<u8>) -> Result<(), Error> {
    let now = SystemTime::now();

    match env::var(DISABLE_SSL_VERIFY) {
        Ok(var) if var == "true" => {
            curl.ssl_verify_host(false)?;
            curl.ssl_verify_peer(false)?;
        }
        _ => (),
    };

    curl.post_field_size(msg.len() as u64)?;
    let mut transfer = curl.transfer();

    transfer.read_function(|buf| Ok(msg.read(buf).unwrap_or(0)))?;
    transfer.write_function(|new_data| {
        result.extend_from_slice(new_data);
        Ok(new_data.len())
    })?;
    transfer.perform()?;

    if let Ok(var) = env::var(DEBUG) {
        if var == "true" {
            let el = now.elapsed().unwrap();
            println!(
                "http post callback_url - {}.{}",
                el.as_secs(),
                el.as_millis()
            );
        }
    }
    Ok(())
}

//-> Result<(), Error>
pub fn api(c_info: &mut ConversationInfo, msg: &[u8]) {
    let curl = match &mut c_info.curl {
        Some(val) => val,
        None => return,
    };

    let mut result = Vec::new();
    if let Err(err) = format_and_transfer(curl, msg, &mut result) {
        match env::var(DEBUG) {
            Ok(ref var) if var == "true" => println!("fail to send msg {:?}", err),
            _ => (),
        };
        return;
    };
}
