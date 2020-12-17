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

export const SUBMIT = '/';

export const isBlankString = (event, value, field) => {
  if (!value.replace(/\s/g, '').length) {
    event.preventDefautl()
    alert(`${field} can't be empty`);
  }
};

export const genJsonPayload = payload => {
  let value = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  };
  return value;
};

const submit = async e => {
  let name = document.getElementById('name').value;
  isBlankString(e, name, 'Name');

  let registration_number = document.getElementById('email').value;
  isBlankString(e, registration_number, 'Registration number');

  let email = document.getElementById('email').value;
  isBlankString(e, email, 'email');

  console.log(`from signup: PoW: ${pow}`);

};

let form = document.getElementById('form');

form.addEventListener('submit', submit, true);
