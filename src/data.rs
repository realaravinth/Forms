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

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Data {
    pub db_pool: PgPool,
}

impl Data {
    #[cfg(not(tarpaulin_include))]
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let pool_size: u32 = env::var("DATABASE_POOL")
            .expect("DATABASE_POOL is not set in .env file")
            .parse()
            .expect("Unable to parse DATABASE_POOL into u32");
        let db_pool = PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(&database_url)
            .await
            .expect("Unable to form database pool");

        Data { db_pool }
    }
}
