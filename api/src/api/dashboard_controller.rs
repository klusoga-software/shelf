use crate::api::dashboard_data::DashboardDataBuilder;
use crate::api::models::CreateDashboardRequest;
use crate::auth::User;
use crate::repository::cargo_repository::CargoRepository;
use crate::repository::dashboards_repository::DashboardsRepository;
use crate::repository::models::Dashboard;
use actix_web::web::Data;
use actix_web::{get, post, Error, HttpResponse, Responder, Scope};
use std::sync::Arc;

pub fn dashboard_controller() -> Scope {
    Scope::new("/dashboard")
        .service(get_dashboard)
        .service(create_dashboard)
        .service(receive_dashboard_data)
}

#[post("")]
async fn create_dashboard(
    user: User,
    state: Data<DashboardsRepository>,
    request: actix_web::web::Json<CreateDashboardRequest>,
) -> Result<impl Responder, Error> {
    state
        .set_dashboard(Dashboard {
            id: None,
            user_id: user.claims.sub,
            tiles: request.into_inner().tiles,
        })
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().finish())
}

#[get("")]
async fn get_dashboard(
    user: User,
    state: Data<DashboardsRepository>,
) -> Result<impl Responder, Error> {
    match state
        .get_dashboard_by_user_id(&user.claims.sub)
        .await
        .map_err(actix_web::error::ErrorBadRequest)?
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(dashboard) => Ok(HttpResponse::Ok().json(dashboard.tiles)),
    }
}

#[get("/data")]
async fn receive_dashboard_data(
    _user: User,
    state: Data<CargoRepository>,
) -> Result<impl Responder, Error> {
    let mut dashboard_builder = DashboardDataBuilder::new();

    let state = Arc::new(state);

    let count_state = Arc::clone(&state);

    let count_handler = tokio::spawn(async move {
        let count = match count_state.get_repo_count().await {
            Ok(count) => count,
            Err(err) => return Err(err),
        };

        Ok(count)
    });

    let size_state = Arc::clone(&state);

    let size_handler = tokio::spawn(async move {
        let count = match size_state.get_crates_size().await {
            Ok(count) => count,
            Err(err) => return Err(err),
        };

        Ok(count)
    });

    let count_result = match count_handler.await {
        Ok(Ok(count)) => count,
        Ok(Err(err)) => return Err(actix_web::error::ErrorInternalServerError(err)),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err)),
    };

    let size_result = match size_handler.await {
        Ok(Ok(count)) => count,
        Ok(Err(err)) => return Err(actix_web::error::ErrorInternalServerError(err)),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err)),
    };

    dashboard_builder
        .repo_count(count_result)
        .storage(size_result);

    Ok(HttpResponse::Ok().json(dashboard_builder.build()))
}
