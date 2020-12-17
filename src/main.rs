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
use sqlx::PgPool;

mod data;
mod error;
mod form;
mod routes;

use data::Data;

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use clap::{App, Arg};

    pretty_env_logger::init();

    let matches = App::new("Shuttlecraft Forms")
        .version("0.1")
        .author("Aravinth Manivannan <realaravinth@batasense.net>")
        .about("Web-based data aggregation")
        .arg(
            Arg::with_name("migrate")
                .short("-m")
                .long("--migrate")
                .help("Database migrations")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present("migrate") {
        migrate().await.unwrap();
    } else {
        start_server().await.unwrap();
    }
    Ok(())
}

pub async fn migrate() -> std::result::Result<(), sqlx::Error> {
    let app_data = Data::new().await;

    sqlx::query!( " CREATE TABLE responses ( name VARCHAR(64) NOT NULL, email_id VARCHAR(40) NOT NULL UNIQUE, registration_number VARCHAR(30) NOT NULL UNIQUE, uuid  VARCHAR(30) NOT NULL UNIQUE PRIMARY KEY
) ",
    )
    .fetch_one(&app_data.db_pool)
    .await?;
    Ok(())
}

async fn start_server() -> std::io::Result<()> {
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
