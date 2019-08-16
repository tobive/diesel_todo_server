# TODO SERVER WITH ROCKET AND DIESEL

Implementation of **RUST** class homework on 8/8/2019 with Diesel.

## How to install

Make sure Postgres is installed and running. Create `.env` file to set database url to connect to Postgres.

`echo DATABASE_URL=postgres://user:password@localhost/rust-web-with-rocket > .env`

Create the database from diesel by running:

`diesel setup`

Start the server using:

`cargo run`

## Example

_POST_

Post json object containing fields `title` and `content`

`curl -d '{"title":"three","content":"content three"}' -H "Content-Type:application/json" -X POST http://localhost:8000/api/post`

_GET_

Get a list of all todos or by todo title

`curl http://localhost:8000/api/list`

`curl http://localhost:8000/api/list?query=three`

_DELETE_

Delete from list by todo id

`curl -X DELETE http://localhost:8000/api/delete?query=1`
