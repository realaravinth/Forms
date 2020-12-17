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

use crate::error::ServiceResult;
use crate::form::FormData;

#[post("/")]
async fn index(form: web::Form<FormData>) -> ServiceResult<impl Responder> {
    let msg = format!("Welcome {}!", form.email_id);
    Ok(HttpResponse::Ok().body(&msg))
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(hello);
}
