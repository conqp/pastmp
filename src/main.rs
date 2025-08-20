use basic_authorization::{AuthenticationError, BasicAuthorization};
use record::Record;
use rocket::serde::json::Json;
use rocket::{State, delete, get, launch, post, routes};
use settings::Settings;

mod accounts;
mod basic_authorization;
mod record;
mod records;
mod settings;

const SETTINGS: &str = "/etc/pastmp.json";

#[post("/", data = "<data>")]
fn upload(
    state: &State<Settings>,
    basic_auth: BasicAuthorization,
    data: Vec<u8>,
) -> Result<Json<usize>, AuthenticationError> {
    basic_auth.validate(&state.accounts, &state.hasher)?;
    state.records.remove_old_entries();
    let id = state.records.insert(data.into_boxed_slice());
    Ok(id.into())
}

#[get("/<id>")]
fn download(
    state: &State<Settings>,
    basic_auth: BasicAuthorization,
    id: usize,
) -> Result<Option<Box<[u8]>>, AuthenticationError> {
    basic_auth.validate(&state.accounts, &state.hasher)?;

    let Some(record) = state.records.get(id) else {
        return Ok(None);
    };

    Ok(Some(record.into_content()))
}

#[delete("/<id>")]
fn remove(
    state: &State<Settings>,
    basic_auth: BasicAuthorization,
    id: usize,
) -> Result<(), AuthenticationError> {
    basic_auth.validate(&state.accounts, &state.hasher)?;
    state.records.remove(id);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Settings::load(SETTINGS).expect("Failed to load settings."))
        .mount("/", routes![upload, download, remove])
}
