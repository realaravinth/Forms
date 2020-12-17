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

#[get("/")]
async fn get_form() -> ServiceResult<impl Responder> {
    let body = FormData::default()
        .render_once()
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(post_form);
    cfg.service(get_form);
}
