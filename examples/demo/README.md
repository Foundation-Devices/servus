servus-demo
============

Simple example showing how to utilize `servus`. This example includes database connectivity with Postgres and
shared state for handler functions.

# Setup

Prior to running the demo, we need to install some CLI tools and setup a local database.

```sh
cargo install sqlx-cli

# you may already have these installed!
cargo install hurl
cargo install just
```

**Also, make sure you have `docker` and `docker-compose` installed.**

Create a file `.env` and add,

```
DATABASE_URL=postgres://demo:demo@localhost/demo
```

# Running

In one terminal window, start the postgres instance and run migrations,

```sh
just examples/demo/setup-db
```

> **NOTE** You will get a compiler error from `sqlx` until the postgres instance is running and the migrations are applied
> with the command above. This is due to the nature of `sqlx`, as it will statically check the validity of queries at compile time.
> This can be adjusted using an "offline" mode, but it's not necessary for this demo. It's more useful during CI builds.

Then, run the demo server,

```sh
cargo run --example demo -- --servus-database-url postgres://demo:demo@localhost/demo
```

In another window, make requests against the server. One will `POST` messages, and the other will `GET` all of them.

```sh
just examples/demo/post-message "zach" "buy a passport!"
just examples/demo/post-message "igor" "use envoy!"

just examples/demo/get-messages
```
