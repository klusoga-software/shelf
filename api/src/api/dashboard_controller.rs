use crate::api::models::CreateDashboardRequest;
use crate::auth::User;
use crate::repository::dashboards_repository::DashboardsRepository;
use crate::repository::models::Dashboard;
use actix_web::web::Data;
use actix_web::{post, Error, HttpResponse, Responder, Scope};

pub fn dashboard_controller() -> Scope {
    Scope::new("/dashboard").service(create_dashboard)
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
