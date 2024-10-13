# Rust Microservice Template

I needed a template to quickly start a new Rust Rocket microservice with Postgres and GraphQL. I don't like spending a lot of time with devops stuff, so I created this template to have a good starting point. I will attempt to keep stable compatibility with future versions of the tools used.

## Prerequisites

- Docker
- Docker Compose

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Setting up the environment

1. Clone the repository:
   ```
   git clone https://github.com/nklnkl/rust-microservice-template.git
   cd rust-microservice-template
   ```

2. Create a `.env` file in the root directory with the following content:
   ```
   DB_NAME=your_project_name
   DB_PASSWORD=your_secure_password
   ```
   Replace `your_secure_password` with a strong password.

### Running the application

#### Development mode

To run the project in development mode with live reloading:

##### Build only

```
docker-compose -f docker-compose.yml -f docker-compose.dev.yml build
```

##### Run only

```
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

##### Build and run

```
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up --build
```

This will:
- Build the Docker images
- Start the application and database containers
- Enable live code reloading
- Expose the database port for direct access

The application will be available at `http://localhost:8000`.

#### Production mode

To run the project in production mode:
```
docker-compose up --build
```

This uses the production settings defined in `docker-compose.yml`.

## API Endpoints

- `GET /`: Returns "Hello, Rust Rocket Microservice Template!"
- `GET /graphql`: GraphiQL interface
- `POST /graphql`: GraphQL endpoint

## Running Tests

To run the tests:
```
docker-compose run app cargo test
```

## Development

The development setup includes hot-reloading. When you make changes to your Rust files, the application will automatically recompile and restart.

## Deployment

Add information about how to deploy this on a live system.

## Built With

- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/)
- [Juniper](https://github.com/graphql-rust/juniper)
- [Diesel](https://diesel.rs/)
- [PostgreSQL](https://www.postgresql.org/)
- [Docker](https://www.docker.com/)

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/nklnkl/rust-microservice-template/tags).

## Authors

- Niko - *Initial work* - [nklnkl](https://github.com/nklnkl)

See also the list of [contributors](https://github.com/nklnkl/rust-microservice-template/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

- Hat tip to anyone whose code was used
- Inspiration
- etc
