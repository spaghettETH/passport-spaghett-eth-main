/*
* Importing base modules
* libs: contains all the libraries used in the application
* modules: contains all the application specific modules
*/

mod libs;
mod modules;

/*
* Importing and running Rocket
*/

#[macro_use]
extern crate rocket;
extern crate argon2;
extern crate dotenv;

use dotenv::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(libs::db::init())
        .attach(libs::cors::CORS)
        .mount(
            "/".to_owned(),
            routes![
                // Basic routes
                libs::index,
                libs::cors::all_options,
                crate::modules::checks::version,
                crate::modules::checks::status,
                // Checkpoints routes
                crate::modules::checkpoints::routes::get_challenge,
                crate::modules::checkpoints::routes::sign_to_checkpoint,
                crate::modules::checkpoints::routes::collected_checkpoints,
                crate::modules::checkpoints::routes::get_checkpoints_for_event,
                crate::modules::checkpoints::routes::create_new_checkpoint,
                crate::modules::checkpoints::routes::update_existing_checkpoint,
                crate::modules::checkpoints::routes::get_leaderboard,
                crate::modules::checkpoints::routes::get_checkpoint_metadata,
                crate::modules::checkpoints::routes::get_checkpoint_id,
                crate::modules::checkpoints::routes::process_unsigned_checkpoints,
                // Users routes
                crate::modules::admins::routes::create_user_session,
                crate::modules::admins::routes::verify_user_session,
                // Passport routes
                crate::modules::passport::routes::get_passport,
                crate::modules::passport::routes::req_mint_passport,
                crate::modules::passport::routes::set_user_profile,
                crate::modules::passport::routes::get_profile,
                crate::modules::passport::routes::sync_passports
            ],
        )
        .register(
            "/",
            catchers![
                libs::not_found,
                libs::general_error,
                libs::malformed_request
            ],
        )
}
