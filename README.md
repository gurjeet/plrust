# PL/Rust: A Trusted Procedural Language Handler

PL/Rust is a loadable procedural language that enables writing PostgreSQL functions in the Rust programming
language. These functions are compiled to native machine code. Unlike other procedural languages, PL/Rust functions
are not interpreted.

The primary advantages of PL/Rust include writing natively-compiled functions to achieve the absolute best performance,
access to Rust's large development ecosystem, and Rust's compile-time safety guarantees.

PL/Rust provides access to Postgres' Server Programming Interface (SPI) including dynamic queries, prepared
statements, and cursors. It also provides safe Rust types over most of Postgres built-in data types, including (but
not limited to), TEXT, INT, BIGINT, NUMERIC, FLOAT, DOUBLE PRECISION, etc.

On x86_64 and aarch64 Linux systems PL/Rust can be a "trusted" procedural language, assuming the proper compilation
requirements are met. On other systems, it is perfectly usable as an "untrusted" language but cannot provide the
same level of safety guarantees.

# Learn More

PL/Rust's documentation, an ongoing project, can be found at https://tcdi.github.io/plrust.  Also see the 
[`plrust-trusted-pgx`](https://docs.rs/plrust-trusted-pgx/latest/plrust_trusted_pgx/) Rust documentation.

# Join our Community

The PL/Rust team at [TCDI](https://www.tcdi.com/) manages a Discord server where we discuss PL/Rust and related technologies
such as [`pgx`](https://github.com/tcdi/pgx), Rust, and Postgres.  Feel free to join:  https://discord.gg/mHKrj55zyh

# Quick Example

An example PL/Rust function:

```sql
psql> CREATE FUNCTION add_two_numbers(a NUMERIC, b NUMERIC) RETURNS NUMERIC STRICT LANGUAGE plrust AS $$
    Ok(Some(a + b))
$$;

psql> SELECT add_two_numbers(2, 2);
add_two_numbers 
-----------------
               4
```

PL/Rust itself is a [`pgx`](https://github.com/tcdi/pgx)-based Postgres extension.  Furthermore,  `LANGUAGE
plrust` functions are themselves mini-pgx extensions. `pgx`is a generalized framework for developing Postgres extensions 
with Rust.  Like this project, `pgx` is developed by [TCDI](https://www.tcdi.com).

The following sections discuss PL/Rusts safety guarantees, configuration settings, and installation instructions.


# Installing PL/Rust

Installing PL/Rust and especially `postgrestd` requires a normal installation of Rust via
[`rustup`](https://rustup.rs) and for the relevant locations to be writeable on the building host.
See the [Install PL/Rust](https://tcdi.github.io/plrust/install-plrust.html)
section of the documentation for notes on installing PL/Rust and its dependencies.



## Cross Compilation Support

See the
[Cross compliation](https://tcdi.github.io/plrust/install-cross-compile.html)
section of the documentation for cross-compilation details.



## Configuration

See the [PostgreSQL Configuration](https://tcdi.github.io/plrust/config-pg.html)
section of the documentation for notes on configuring PL/Rust in
`postgresql.conf`.


----


## Lints

See the [Lints section](https://tcdi.github.io/plrust/config-lints.html)
of the documentation.


## Environment Variables

See the [Environment variables section](https://tcdi.github.io/plrust/config-env-var.html)
of the documentation.



# Quickly Getting Started

To quickly evaluate PL/Rust from this repository...

First, install and initialize the required build environment tools:

```bash
$ cargo install cargo-pgx --locked
$ cargo pgx init
```

Then clone this repository and build/run PL/Rust once to complete the `cargo-pgx` environment initialization:

```bash
$ git clone https://github.com/tcdi/plrust.git
$ cd plrust

# build plrustc, our custom rustc driver and copy it to ~/.cargo/bin
$ cd plrustc && ./build.sh    
$ cp ../build/bin/plrustc ~/.cargo/bin

# build and run plrust itself
$ cd ../plrust/plrust
$ cargo pgx run pg14 --release

# which drops you into a psql shell.  just \quit it for now
psql> \q
```

Apply the required `postgresql.conf` configuration:

```bash
$ SCRATCH_DIR=${HOME}/plrust-scratch
$ mkdir -p ${SCRATCH_DIR}
$ cat <<-EOF >> ~/.pgx/data-14/postgresql.conf
shared_preload_libraries = 'plrust'
plrust.work_dir = '${SCRATCH_DIR}'
EOF
```

Finally, run it for real and start writing functions!

```bash
$ cargo pgx run pg14
psql> CREATE EXTENSION plrust;
psql> CREATE FUNCTION add_two_numbers(a NUMERIC, b NUMERIC) RETURNS NUMERIC STRICT LANGUAGE plrust AS $$
    Ok(Some(a + b))
$$;

psql> SELECT add_two_numbers(2, 2);
add_two_numbers 
-----------------
               4
```


# Other Notes

In the Postgres world it seems common for procedural languages to have two styles, "trusted" and "untrusted".  The consensus is to name those as "lang" and "langu", respectively -- where the "u" is supposed to represent "untrusted" (see "plperl" v/s "plperlu" for example).

PL/Rust does not do this.  The only thing that Postgres uses to determine if a language handler is considered "trusted" is if it was created using `CREATE TRUSTED LANGUAGE`.  It does not inspect the name.

PL/Rust stores user functions in `pg_catalog.pg_proc`'s `prosrc` field as a complex json structure where the compiled 
function is a compressed, base64 encoded string, with the key-value pairs mapping each target tuple to compiled object code.

As such, compiling a function with an "untrusted" version of PL/Rust, then installing the "trusted" version and trying to run that function will fail -- "trusted" and "untrusted" are considered different compilation targets and are not compatible with each other, even if the underlying hardware is exactly the same.

This does mean that it is not possible to install both "trusted" and "untrusted" versions of PL/Rust on the same Postgres database cluster.

In the future, as `postgrestd` is ported to more platforms, we will seriously consider having both `plrust` and `plrustu`.  Right now, since "trusted" is only possible on Linux `x86_64`/`aarch64`, our objective is to drive production installations to be "trusted", while allowing non-Linux developers the ability to use `LANGUAGE plrust` too.


# Security Notice

Please read the [Security](SECURITY.md) for directions on reporting a potential security issue.

# License

PL/Rust is licensed under "The PostgreSQL License", which can be found [here](LICENSE.md).
