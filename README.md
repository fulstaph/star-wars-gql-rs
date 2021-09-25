# Star Wars GraphQL API written in Rust

My take on standart graphql api example using async-graphql & sqlx

## Running service

- run `make build-dev`
- run `make run-dev-env`
- run `make dev-migration-up` (optional: populate db with test data in `test-data.sql`)
- run `make run-app`
- go to `http://0.0.0.0:8080/graphql/playground` in your browser
