//! Pastebin service with basic HTTP authentication, that stores data in RAM.

use basic_authorization::BasicAuthorization;
use error::Error;
use record::Record;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{State, delete, get, launch, post, routes};
use settings::Settings;

mod accounts;
mod basic_authorization;
mod error;
mod record;
mod records;
mod settings;

const SETTINGS: &str = "/etc/pastmp.json";

#[post("/", data = "<data>")]
fn upload(
    state: &State<Settings>,
    basic_auth: BasicAuthorization,
    data: Vec<u8>,
) -> Result<Json<usize>, Error> {
    basic_auth.validate(&state.accounts, &state.hasher)?;
    info!("Adding record of size {} bytes.", data.len());
    info!(
        "Total memory used before cleanup: {} bytes",
        state.records.size()
    );
    state.records.remove_old_entries();
    info!(
        "Total memory used after cleanup: {} bytes",
        state.records.size()
    );
    let id = state.records.insert(data.into_boxed_slice());
    info!("Total memory used: {} bytes", state.records.size());
    Ok(id.into())
}

#[get("/<id>")]
fn download(
    state: &State<Settings>,
    basic_auth: BasicAuthorization,
    id: usize,
) -> Result<Box<[u8]>, Error> {
    basic_auth.validate(&state.accounts, &state.hasher)?;

    let Some(content) = state.records.get(id) else {
        return Err(Error::NotFound);
    };

    Ok(content)
}

#[delete("/<id>")]
fn remove(state: &State<Settings>, basic_auth: BasicAuthorization, id: usize) -> Result<(), Error> {
    basic_auth.validate(&state.accounts, &state.hasher)?;
    info!(
        "Total memory used before removal: {} bytes",
        state.records.size()
    );

    if state.records.remove(id).is_none() {
        return Err(Error::NotFound);
    }

    info!(
        "Total memory used after removal: {} bytes",
        state.records.size()
    );
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Settings::load(SETTINGS).expect("Failed to load settings."))
        .mount("/", routes![upload, download, remove])
}
