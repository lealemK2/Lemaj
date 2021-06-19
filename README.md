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
- The database is [PostgreSQL](https://www.postgresql.org "PostgreSQL Database").
- `HTML`, `CSS` and `Javascript` are the front-end programming languages.
The tools used to build the website are available for all major operating systems. 

## How to install?

### Install dependencies

Install Rust programming language, you can get it [here](https://www.rust-lang.org/tools/install).
Install PostgreSQL database, you can get it [here](https://www.postgresql.org/download).
Add environment variables, you can use `.env` file. It could look like
```
PG__DBNAME=lemaj_dev
PG__USER=lemaj
PG__PASSWORD=lemajpass
PG__HOST=localhost
MAX_PAGE_SIZE=50
```

## Contribution

Checkout the [To-do](TODO.md "The To-do list") list

## License

Lemaj is licensed under [MIT License](https://opensource.org/licenses/MIT "MIT License")