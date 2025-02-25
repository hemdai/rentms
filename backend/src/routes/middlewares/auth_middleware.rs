use crate::utils::{
    api_response::ApiResponse,
    app_state,
    jwt::{check_token_in_db, decode_jwt},
};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
    web, Error, HttpMessage,
};

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get(AUTHORIZATION);
    if auth.is_none() {
        return Err(Error::from(ApiResponse::new(
            401,
            "Unauthorised".to_string(),
        )));
    }
    let token = auth
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "")
        .to_owned();
    let app_state = req
        .app_data::<web::Data<app_state::AppState>>()
        .ok_or_else(|| Error::from(ApiResponse::new(500, "App state missing".to_string())))?;

    let valid_token_claim = check_token_in_db(token.clone(), app_state.clone()).await?;

    if valid_token_claim.is_valid {
        req.extensions_mut().insert(valid_token_claim);
        return next
            .call(req)
            .await
            .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())));
    }
    print!("Decoding the token from here ");
    let claim =
        decode_jwt(token).map_err(|err| Error::from(ApiResponse::new(401, err.to_string())))?;
    req.extensions_mut().insert(claim);
    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}
