use crate::account::{Account, AuthenticationError};
use record::Record;
use rocket::serde::json::Json;
use rocket::{State, delete, get, launch, post, routes};
use settings::Settings;

mod account;
mod accounts;
mod record;
mod records;
mod settings;

const SETTINGS: &str = "/etc/pastmp.json";

#[post("/", data = "<data>")]
fn upload(
    state: &State<Settings>,
    account: Account,
    data: Vec<u8>,
) -> Result<Json<usize>, AuthenticationError> {
    account.validate(&state.accounts, &state.hasher)?;
    let id = state.records.insert(data.into_boxed_slice());
    Ok(id.into())
}

#[get("/<id>")]
fn download(
    state: &State<Settings>,
    account: Account,
    id: usize,
) -> Result<Option<Box<[u8]>>, AuthenticationError> {
    account.validate(&state.accounts, &state.hasher)?;

    let Some(record) = state.records.get(&id) else {
        return Ok(None);
    };

    Ok(Some(record.into_content()))
}

#[delete("/<id>")]
fn remove(state: &State<Settings>, account: Account, id: usize) -> Result<(), AuthenticationError> {
    account.validate(&state.accounts, &state.hasher)?;
    state.records.remove(&id);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Settings::load(SETTINGS).expect("Failed to load settings."))
        .mount("/", routes![upload, download, remove])
}
