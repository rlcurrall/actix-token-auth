# Actix Web & SPA Token Authentication

This repository is an example for creating an Actix Web API that is consumed by a Vue single page application.

The implementation used is inspired by [Laravel Sanctum](https://github.com/laravel/sanctum).

## Running the Application

### 1. Add entry to hosts file for local development

This is certainly not a requirement, but just to demonstrate that if you wish to have the authentication token
stored on a http-only cookie, the API and the SPA need to be running on the same domain, though they can exist
on different sub-domains.

Add the following lines to your `/etc/hosts` file:
```
127.0.0.1       api.my-app.test
127.0.0.1       web.my-app.test
```

### 2. Create `.env` Files

Example files have been provided, you can run the following command to copy them into the necessary files
Modify these files as you see fit.

```bash
cp .env.example .env && cp web/.env.example web/.env
```

### 3. Start Actix Web API

To run the application in development mode you can simply run:
```bash
cargo run
```

Though, if you would like to compile the application for release, run the following command:
```bash
cargo build --release && ./target/release/rcs
```

### 4. Start Vue App

Serve the Vue app by running the following command:
```bash
cd web && npm run dev
```

### 5. Open Browser

Once you have completed all the above steps, you can now open your browser to http://web.my-app.test:3000 to be able to interact with the application.
