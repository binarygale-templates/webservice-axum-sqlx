# webservice-axum-sqlx

This is a template project for a webservice using `axum` and `sqlx` to connect to a PostgreSQL database. It is based on the [`webservice-axum` template](https://github.com/binarygale-templates/webservice-axum), and adds additional things on top of it.

## Features

On top of everything in `webservice-axum`, this adds:

- a globally available DB Pool that automatically scales to the number of CPU cores to match the default Tokio scaling.
- automatic migrations on startup.
- a database trigger to automatically update an `updated_at` column, see the initial database scheme. This was inspired by Diesel. :)
- a readyness check that fails if the database connection is unavailable.

## Linting

In addition to the `pre-commit` setup from `webservice-axum`, this template also makes sure that the offline query data fro SQLx is uptodate.
