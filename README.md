# Star Wars GraphQL API written in Rust

My take on standart graphql api example using async-graphql & sqlx

## Running service

- run `make build-dev`
- run `make run-dev-env`
- run `make dev-migration-up` (optional: populate db with test data in `test-data.sql`)
- run `make run-app`
- go to `http://0.0.0.0:8080/graphql/playground` in your browser

## Schema (subject to change)
```graphql
# Directs the executor to query only when the field exists.
directive @ifdef on FIELD

type Character {
  id: ID!
  name: String!
  race: Race!
  starship: Starship
  friends: [Character!]
}

type Filmmaker {
  id: ID!
  firstName: String!
  lastName: String!
  profession: Profession!
}

type Movie {
  id: ID!
  title: String!
  director: Filmmaker!
  scriptwriter: Filmmaker!
  producer: Filmmaker!
  releaseDate: NaiveDate!
}

scalar NaiveDate

type Planet {
  id: ID!
  name: String!
}

enum Profession {
  DIRECTOR
  SCRIPTWRITER
  PRODUCER
  CINEMATOGRAPHER
}

type Query {
  starship(id: ID!): Starship
  planet(id: ID!): Planet
  character(id: ID!): Character
  movie(id: ID!): Movie
}

enum Race {
  HUMAN
  DROID
  WOOKIE
}

type Starship {
  id: ID!
  name: String!
  class: String!
}
```
