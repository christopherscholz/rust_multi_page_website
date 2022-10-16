Personal static Multi-Page-Website for Christopher Scholz build with [Rocket](https://rocket.rs/).
* Using Tera Templates via `rocket_dyn_templates::Template` to create the home and 404 page
* Serving Static Files via `rocket::fs::FileServer`
* Tests for home page, impressum page, 404 page and static files response

The app can be run locally or via docker.

For the `local` setup run
```
cargo test
cargo run
```

For the `docker` setup run
```
docker build . -t christopher-scholz/multi_page_website
docker run -p 8000:8000 -d --rm --name multi_page_website christopher-scholz/multi_page_website
```
