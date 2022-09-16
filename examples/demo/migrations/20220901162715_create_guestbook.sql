-- SPDX-FileCopyrightText: 2022 Foundation Devices Inc. <hello@foundationdevices.com>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

-- Add migration script here
CREATE TABLE IF NOT EXISTS guestbook (
    id      SERIAL PRIMARY KEY,
    author  VARCHAR(32) NOT NULL,
    message VARCHAR(280) NOT NULL
);
