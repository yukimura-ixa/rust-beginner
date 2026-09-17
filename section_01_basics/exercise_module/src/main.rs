mod config;
mod models;
mod services;
fn main() {
    config::init();
    models::user::show();
    models::product::show();
    services::auth::auth();
    services::payment::pay();
    
}
