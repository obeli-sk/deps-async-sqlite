//! # async-sqlite
//!
//! A library to interact with sqlite from an async context.
//!
//! This library is tested on both [tokio](https://docs.rs/tokio/latest/tokio/)
//! and [async_std](https://docs.rs/async-std/latest/async_std/), however
//! it should be compatible with all async runtimes.
//!
//! ## Usage
//!
//! A `Client` represents a single background sqlite3 connection that can be called
//! concurrently from any thread in your program.
//!
//! To create a sqlite client and run a query:
//!
//! ```rust
//! use obeli_sk_deps_async_sqlite::{ClientBuilder, JournalMode};
//!
//! # async fn run() -> Result<(), obeli_sk_deps_async_sqlite::Error> {
//! let client = ClientBuilder::new()
//!                 .path("/path/to/db.sqlite3")
//!                 .journal_mode(JournalMode::Wal)
//!                 .open()
//!                 .await?;
//!
//! let value: String = client.conn(|conn| {
//!     conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))
//! }).await?;
//!
//! println!("Value is: {value}");
//! # Ok(())
//! }
//! ```
//!
//! A `Pool` represents a collection of background sqlite3 connections that can be
//! called concurrently from any thread in your program.
//!
//! To create a sqlite pool and run a query:
//!
//! ```rust
//! use obeli_sk_deps_async_sqlite::{JournalMode, PoolBuilder};
//!
//! # async fn run() -> Result<(), obeli_sk_deps_async_sqlite::Error> {
//! let pool = PoolBuilder::new()
//!               .path("/path/to/db.sqlite3")
//!               .journal_mode(JournalMode::Wal)
//!               .open()
//!               .await?;
//!
//! let value: String = pool.conn(|conn| {
//!     conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))
//! }).await?;
//!
//! println!("Value is: {value}");
//! # Ok(())
//! # }
//! ```
//!
//! ## Cargo Features
//!
//! This library tries to export almost all features that the underlying
//! [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) library contains.
//!
//! A notable difference is that the `bundled` feature is **enabled** by default,
//! but can be disabled with the following line in your Cargo.toml:
//!
//! ```toml
//! async-sqlite = { version = "*", default-features = false }
//! ```

pub use rusqlite;

mod client;
mod error;
mod pool;

pub use client::{Client, ClientBuilder, JournalMode};
pub use error::Error;
pub use pool::{Pool, PoolBuilder};
