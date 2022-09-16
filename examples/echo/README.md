<!--
SPDX-FileCopyrightText: 2022 Foundation Devices Inc. <hello@foundationdevices.com>

SPDX-License-Identifier: AGPL-3.0-or-later
-->

servus-echo
===========

This example is a simple "echo" server, where the given message in the path is echoed back to the user.

Run with,

```
cargo run --example echo
```

### Echo Route

```
curl -i http://localhost:8000/echo/test
```
