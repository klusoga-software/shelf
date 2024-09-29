#[macro_export]
macro_rules! log_error_and_responde {
    ($error:expr) => {{
        log::error!("{}", $error.to_string());

        HttpResponse::InternalServerError().body($error.to_string())
    }};
}
