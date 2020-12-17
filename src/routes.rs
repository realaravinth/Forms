/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::{get, post, web, web::ServiceConfig, HttpResponse, Responder};
use sailfish::TemplateOnce;

use crate::error::*;
use crate::form::{FormData, Name};

#[post("/")]
async fn post_form(form: web::Form<FormData>) -> ServiceResult<impl Responder> {
    let body = Name::new(&form.name)
        .render_once()
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[cfg(not(tarpaulin_include))]
#[get("/")]
async fn get_form() -> ServiceResult<impl Responder> {
    let body = FormData::default()
        .render_once()
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[cfg(not(tarpaulin_include))]
pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(post_form);
    cfg.service(get_form);
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{http::header, test, App};
    use serde_urlencoded;

    #[actix_rt::test]
    async fn post_form_works() {
        let mut app = test::init_service(App::new().configure(services)).await;
        let form_data = FormData {
            name: "ASDF".into(),
            email_id: "a@a.com".into(),
            registration_number: "asdf".into(),
        };

        let payload = serde_urlencoded::to_string(form_data).unwrap();

        let resp = test::call_service(
            &mut app,
            test::TestRequest::post()
                .uri("/")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .set_payload(payload)
                .to_request(),
        )
        .await;
        assert!(resp.status().is_success());
    }
}
