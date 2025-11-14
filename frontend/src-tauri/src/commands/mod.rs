pub mod backend;
pub mod health;
pub mod files;
pub mod connection;
pub mod auth;

pub use backend::{cleanup_backend, start_backend};
pub use health::check_backend_health;
pub use files::{add_opened_file, clear_opened_files, get_opened_files};
pub use connection::{
    get_connection_config,
    is_first_launch,
    set_connection_mode,
    test_server_connection,
};
pub use auth::{
    clear_auth_token,
    clear_user_info,
    get_auth_token,
    get_user_info,
    save_auth_token,
    save_user_info,
};
