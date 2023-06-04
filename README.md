# Linux Global Keylogger

## Purpose

- This Linux software is a binary executable that will globally log all connected devices key inputs, and store them in a remote MySQL database. **Written in Rust with crates evdev, tokio, and sqlx**.
- Built as a portfolio project, this project demonstrates my ability to utilize async Rust, MySQL, and Linux Systems programming, to make a fast and consistent piece of software 😏.

## Dependencies

1. [rustup](https://rustup.rs/) (Rust toolchain installer)
2. [Mysql Community Server](https://dev.mysql.com/downloads/mysql/)
3. Running a Linux Distro (The only OS specific code here is accessing the device input events).

## Build Guide

1. `git clone {}`, repo_ssh_or_https
2. Start your MySQL server locally or remote. `sudo systemctl start mysqld`
3. Create the keylogger database and key_logs table.

- `mysql -u <your_username_typically_root> -p`
- In MySQL Monitor: `CREATE DATABASE keylogger` && `USE keylogger` && `source <path_to_key_logs.sql_file_in_repository>`

4. Validate in main.rs the MySQL Connection statement is pointing to your localhost or remote and port, and uses the correct password.
5. `cargo build`
6. `sudo ./target/debug/keylogger` (Needs to be run with sudo permissions, so you can't just `cargo run`.)
