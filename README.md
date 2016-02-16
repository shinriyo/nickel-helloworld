# nickel-helloworld

#  Rust + nickel framework + PostgreSQL remplates

## How to run PostgreSQL 
```
postgres -D /usr/local/var/postgres
```

## SPA(Single Page Application) which I refered
http://www.sitepoint.com/creating-crud-app-minutes-angulars-resource/

# API URL
* Create DataBase
`http://localhost:6767/setup/movie`
(caution) don't forget last `/`

## How to Run
`cargo run`

And, later access `http://localhost:6767/movie_app`

## CRUD

* Movie List
GET http://localhost:6767/api/movie/

* Create
POST http://localhost:6767/api/movie/:id

* Read
GET http://localhost:6767/api/movie/:id

* Update
PUT http://localhost:6767/api/movie

* Delete
DELETE http://localhost:6767/api/movie/:id

## Postgres Plugin
postgres = "0.11"
sfackler/rust-postgres

## License

nickel-helloworld [MIT license](http://www.opensource.org/licenses/MIT)

## Author

* [shinriyo](https://github.com/shinriyo) (Lead developer)

