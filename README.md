# Lemaj - website to improve coding skills and compete in contests

```
Development on Lemaj has just started. Everything is in flux
```
## What is Lemaj?

__Lemaj__ aspires to be a website for learning programming, it will have weekly contests where any one can compete. We believe the best way to learn programming is by writing code and __Lemaj__ aims to help in providing a platform to do just that. Think of it as [Leetcode](https://leetcode.com) or [Codeforces](https://codeforces.com) but aimed at beginners and Ethiopian programmers. __Lemaj__ will try to remove barriers to entry as much as possible. The word __Lemaj__ means trainee in [Amharic](https://en.wikipedia.org/wiki/Amharic) and it is written as `ለማጅ`. If you have an idea on how to improve __Lemaj__ please edit the section below (`Ideas on how to improve`) to add what you think and submit a merge request.

### Ideas on how to improve

- Add localization support to help non-English speakers
- Help programmers with getting get a job
- Enable companies with interviewing process
- Organize meet-ups and events, include competetion and prizes
- Tutorials and courses

## Software used to build the website

- [Rust](https://www.rust-lang.org "Rust programming language") is the back-end programming language and we use [Rocket](https://rocket.rs) web framework.
- The database is [PostgreSQL](https://www.postgresql.org "PostgreSQL Database") and we use [Diesel_CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) ORM.
- `HTML`, `CSS` and `Javascript` are the front-end programming languages.
The tools used to build the website are available for all major operating systems. 

## How to install?

### Install dependencies

Install Rust programming language, you can get it [here](https://www.rust-lang.org/tools/install).
Install PostgreSQL database, you can get it [here](https://www.postgresql.org/download). Create a role `lemaj` with password `lemajpass`. Database role/password will be moved out of the repo in the future.
Install Diesel CLI, you can get it here [here](https://crates.io/crates/diesel_cli). For this project we need the `postgres` feature. So use:
```shell
$ cargo install diesel_cli --no-default-features --features postgres
```
Add an environment variable `DATABASE_URL` with value of the database url. You can also use `.env` file:
```shell
$ echo DATABASE_URL=postgres://lemaj:lemajpass@localhost:5432/lemaj_dev > .env
```

### If you face linking errors while installing `diesel cli`:
#### Windows
Add an environment variable `PQ_LIB_DIR` and set it's value to the path to `postgres`'s lib folder (e.g. `C:\Program Files\PostgreSQL\13\lib`). You might also need to add `postgres`'s bin folder to your `Path` if you haven't already

#### Linux (Ubuntu)
Try installing `libpq-dev`

```shell
$ sudo apt-get install libpq-dev
```

## Contribution

Checkout the [To-do](TODO.md "The To-do list") list

## License

Lemaj is licensed under [MIT License](https://opensource.org/licenses/MIT "MIT License")