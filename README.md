# Proyecto de PiscoAcustica
## Run project

In order to run this project you need to have [Rust with Cargo](https://www.rust-lang.org/tools/install) 
installed locally and run the following command in the project directory:
```
cargo run
```

## Deploy to Heroku

In order to deploy to Heroku the following steps must be followed:

1. Fork this repository
2. Create a heroku account and app using the type of dyno of your selection. 
3. Connect the app with the github account that forked this repository and select the forked repository as source.
4. Attach a Postgresql add-on to the dyno following these [instruction](https://devcenter.heroku.com/articles/heroku-postgresql#provisioning-heroku-postgres)
4. Deploy the application.

Currently the application runs in a Docker environment, the ```heroku.yaml``` file is configured to allow for automatic deploys from heroku, however it's reccommended to follow the container registry deployment method of heroku as it has a better compile time and deployment time.
