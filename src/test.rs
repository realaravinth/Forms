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
use std::borrow::Cow;

mod data;
mod error;
mod form;
mod routes;

use data::Data;
use sqlx::PgPool;
use sqlx::{migrate::MigrateError, Error};

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //    use clap::{App, Arg};

    pretty_env_logger::init();

    //    let matches = App::new("Shuttlecraft Forms")
    //        .version("0.1")
    //        .author("Aravinth Manivannan <realaravinth@batasense.net>")
    //        .about("Web-based data aggregation")
    //        .arg(
    //            Arg::with_name("migrate")
    //                .short("-m")
    //                .long("--migrate")
    //                .help("Database migrations")
    //                .takes_value(false),
    //        )
    //        .get_matches();
    //

    let data = Data::new().await;

    if let Err(val) = migrate(&data.db_pool).await {
        if let Error::Database(err) = val {
            println!("{:#?}", err.code());
            if err.code() == Some(Cow::from("42P07")) {
                let _ = drop(&data.db_pool).await;
                let _ = migrate(&data.db_pool).await;
            }
        }
    }

    Ok(())
}

async fn migrate(pool: &PgPool) -> std::result::Result<(), sqlx::Error> {
    sqlx::query!( " CREATE TABLE responses ( name VARCHAR(64) NOT NULL, email_id VARCHAR(40) NOT NULL UNIQUE, registration_number VARCHAR(30) NOT NULL UNIQUE, uuid  VARCHAR(90) NOT NULL UNIQUE PRIMARY KEY
) ",
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

async fn drop(pool: &PgPool) -> std::result::Result<(), sqlx::Error> {
    sqlx::query!("DROP TABLE responses")
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(())
}
