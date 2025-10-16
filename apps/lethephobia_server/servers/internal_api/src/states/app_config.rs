use axum::extract::FromRef;

use super::AppState;

#[derive(Clone)]
pub struct AppConfig;

impl FromRef<AppState> for AppConfig {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.app_config.clone()
    }
}
