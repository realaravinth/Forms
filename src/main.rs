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
use std::env;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use pretty_env_logger;

mod data;
mod error;
mod form;
mod routes;

use data::Data;

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let port: u32 = env::var("PORT")
        .unwrap_or({
            println!("PORT not set, defaulting to 3000");
            "3000".into()
        })
        .parse()
        .expect("PORT should be an integer");

    let app_data = Data::new().await;

    HttpServer::new(move || {
        App::new()
            .configure(routes::services)
            .service(Files::new("/", "./static"))
            .data(app_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
