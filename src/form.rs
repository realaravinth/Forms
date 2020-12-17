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

use ammonia::clean;
use sailfish_macros::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

use crate::error::ServiceResult;

#[derive(Deserialize, Debug, Default, Serialize, TemplateOnce)]
#[template(path = "form.stpl")]
pub struct FormData {
    pub email_id: String,
    pub name: String,
    pub registration_number: String,
}

#[derive(Debug, Default, TemplateOnce)]
#[template(path = "response-saved.stpl")]
pub struct Name<'a> {
    pub name: &'a str,
}

#[derive(Debug, Default, TemplateOnce)]
#[template(path = "error.stpl")]
pub struct ErrorMessage<'a> {
    pub error: &'a str,
}

impl<'a> ErrorMessage<'a> {
    pub fn new(error: &'a str) -> Self {
        ErrorMessage { error }
    }
}

impl<'a> Name<'a> {
    pub fn new(name: &'a str) -> Self {
        Name { name }
    }
}

impl FormData {
    pub fn process(&self) -> ServiceResult<Candidate> {
        let candidate = Candidate::new()
            .set_name(&self.name)
            .set_email(&self.email_id)?
            .set_registration_number(&self.registration_number)
            .build();

        Ok(candidate)
    }

    pub fn empty() -> Self {
        FormData {
            name: " ".into(),
            email_id: " ".into(),
            registration_number: " ".into(),
        }
    }
}

#[derive(Deserialize, Clone, Validate, Serialize, Debug, Default)]
pub struct Candidate {
    uuid: Uuid,
    #[validate(email)]
    email_id: String,
    name: String,
    registration_number: String,
}

impl Candidate {
    fn new() -> Self {
        let mut candidate: Candidate = Default::default();
        candidate.uuid = Uuid::new_v4();
        candidate
    }

    fn set_name<'a>(&'a mut self, name: &str) -> &'a mut Self {
        self.name = clean(name).trim().to_owned();
        self
    }

    fn set_registration_number<'a>(&'a mut self, reg: &str) -> &'a mut Self {
        self.registration_number = clean(reg).trim().to_owned();
        self
    }

    fn set_email<'a>(&'a mut self, email_id: &str) -> ServiceResult<&'a mut Self> {
        self.email_id = email_id.into();
        self.validate()?;
        Ok(self)
    }

    fn build(&mut self) -> Self {
        self.to_owned()
    }

    pub async fn save(&mut self, pool: &PgPool) -> ServiceResult<()> {
        sqlx::query!( "INSERT INTO responses ( name , email_id, registration_number, uuid) VALUES ($1, $2, $3, $4) RETURNING uuid done ", self.name, self.email_id, self.registration_number, self.uuid.to_hyphenated().to_string())

    .fetch_one(pool)
    .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::*;

    #[test]
    fn candidate_builer_works() {
        let candidate = Candidate::new()
            .set_name("aravinth")
            .set_email("batman@we.net".into())
            .unwrap()
            .set_registration_number("19bce7415")
            .build();

        assert_eq!(candidate.name, "aravinth");
        assert_eq!(candidate.registration_number, "19bce7415");
        assert_eq!(candidate.email_id, "batman@we.net");
    }

    #[test]
    fn candidate_email_err() {
        let mut candidate = Candidate::new()
            .set_name("aravinth")
            .set_registration_number("asdasd")
            .build();

        assert_eq!(
            candidate.set_email("batmanwe.net").err(),
            Some(ServiceError::NotAnEmail)
        );
    }

    #[test]
    fn form_data_process_works() {
        let mut form_data = FormData {
            name: "aravinth".into(),
            registration_number: "19bce7415".into(),
            email_id: "batman@me.net".into(),
        };
        let candidate = form_data.process().unwrap();
        assert_eq!(candidate.name, "aravinth");
        assert_eq!(candidate.registration_number, "19bce7415");
        assert_eq!(candidate.email_id, "batman@me.net");

        form_data.email_id = "sdfsf".into();

        assert_eq!(form_data.process().err(), Some(ServiceError::NotAnEmail));
    }
}
