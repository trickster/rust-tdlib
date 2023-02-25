use crate::errors::Result;
use crate::tdjson;
use crate::types::RFunction;

/// A bridge between TDLib and rust-tdlib.
pub trait TdLibClient {
    fn send<Fnc: RFunction>(&self, client_id: tdjson::ClientId, fnc: Fnc) -> Result<()>;
    fn receive(&self, timeout: f64) -> Option<String>;
    fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> Result<Option<String>>;
    fn new_client(&self) -> tdjson::ClientId;
}

#[derive(Clone, Debug, Copy)]
/// Base implementation. See [tdjson](crate::tdjson) for details.
pub struct TdJson;

impl Default for TdJson {
    fn default() -> Self {
        Self
    }
}

impl TdLibClient for TdJson {
    fn send<Fnc: RFunction>(&self, client_id: tdjson::ClientId, fnc: Fnc) -> Result<()> {
        let json = fnc.to_json()?;

        // log::debug!("-client_id: {client_id}-send_raw_json: {} ", json);
        tdjson::send(client_id, &json[..]);
        Ok(())
    }

    fn receive(&self, timeout: f64) -> Option<String> {
        tdjson::receive(timeout)
    }

    fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> Result<Option<String>> {
        let json = fnc.to_json()?;
        Ok(tdjson::execute(&json[..]))
    }

    fn new_client(&self) -> tdjson::ClientId {
        tdjson::new_client()
    }
}

impl TdJson {
    pub fn new() -> Self {
        Self {}
    }
}
