# pg_flagsmith

---

**Source Code**: <a href="https://github.com/KirioXX/pg_flagsmith" target="_blank">https://github.com/KirioXX/pg_flagsmith</a>

---

## Summary

`pg_flagmith` is a wrapper for the [Flagsmith](https://flagsmith.com/) client to query feature flags.

## API

SQL functions:

```sql
-- Check if a feature [flag] is enabled
is_feature_enabled(flagsmith_key string, flag_name string) returns bool
```

and

```sql
-- Check if a feature [flag] is enabled for a specific [identifier]
is_feature_enabled_for_identity(flagsmith_key string, identifier string, flag_name string) returns bool
```

## Try it Out

Spin up Postgres with pg_jsonschema installed in a docker container via `docker-compose up`. The database is available at `postgresql://postgres:password@localhost:5407/app`

## Installation

Requires:

- [pgx](https://github.com/tcdi/pgx)

```shell
cargo pgx run
```

which drops into a psql prompt.

```psql
psql (13.6)
Type "help" for help.

pg_flagsmith=# create extension pg_flagsmith;
CREATE EXTENSION

pg_jsonschema=# select is_feature_enabled('[YOUR_KEY]', '[YOUR_FEATURE_FLAG]');
 is_feature_enabled
---------------------
 t
(1 row)
```

for more complete installation guidelines see the [pgx](https://github.com/tcdi/pgx) docs.

## Prior Art

[flagsmith-rust-client](https://github.com/Flagsmith/flagsmith-rust-client) - The SDK for Rust applications for Flagsmith

[Server Side SDKs](https://docs.flagsmith.com/clients/server-side) - Flagsmith client documentation for servers
