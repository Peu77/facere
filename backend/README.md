## Rust backend with Diesel, Actix-Web and PostgreSQL.

This repository contains a simple backend written in Rust. It uses the [Diesel ORM](https://diesel.rs/) for the database
abstraction and the [Actix-Web](https://actix.rs/) framework for the web server implementation.

The backend uses [PostgreSQL](https://www.postgresql.org/) as the database backend.

### Installation

- First, you need to install Rust on your system. You can find instructions on how to do this on the [official Rust
  website.](https://www.rust-lang.org/tools/install)

- Next, you need to install PostgreSQL. Instructions for this can be found on
  the [official PostgreSQL website.](https://www.postgresql.org/download/)

Clone the repository to your local computer:

```git clone https://github.com/peu77/facere```

Navigate to the repository backend directory and run the following command to install all dependencies:

```cargo build```

### Configure

Create a .env file in the backend directory and add the following lines:

```DATABASE_URL=postgres://username:password@localhost/database_name```

Replace `username`, `password` and ``database_name`` with the appropriate values for your PostgreSQL installation.

### Execute

Run
Start the web server with the following command:

```cargo run```

The web server will run on http://localhost:8080.

### Post

If you find bugs or want to suggest improvements, feel free to contribute. Just create a pull request with your changes.
We will try to review and merge them as soon as possible.