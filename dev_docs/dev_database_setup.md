# Set up a local dev PostgreSQL database 

### 1. Initialize the Database
```console
pg_ctl -D "C:\path_to\db" init
```

### 2. Start PostgreSQL
```console
pg_ctl -D "C:\path_to\db" -l "C:\path_to\db_logs\logfile" start
```

#### Troubleshooting:
If starting PostgreSQL fails, maybe the default 5432 port is already in use by another OS process.
Modify C:\path_to\db\postgresql.conf file to change the PostgreSQL port.
```
# - Connection Settings -

# listen_addresses = 'localhost'		# what IP address(es) to listen on;
					# comma-separated list of addresses;
					# defaults to 'localhost'; use '*' for all
					# (change requires restart)
port = 6543				# (change requires restart)
```

### 3. Create a User and Database
```console
psql -U postgres
postgres=# CREATE ROLE user_dev WITH LOGIN PASSWORD 'user_dev';
postgres=# CREATE DATABASE restaurant_db_dev OWNER user_dev;
postgres=# \q
```
### 4. Set Up sqlx-cli to manage database schema

Install sqlx-cli globally with Cargo
```console
cargo install sqlx-cli --no-default-features --features postgres
```

Initialize the migrations directory (by default, it's a directory called migrations):
```console
sqlx migrate add initial_schema
```
Apply the migration to your database with:
```console
sqlx migrate run
```

Note: output after adding a migration by sqlx crate:
```
Did you know you can embed your migrations in your application binary?
On startup, after creating your database connection or pool, add:

sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

Note that the compiler won't pick up new migrations if no Rust source files have changed.
You can create a Cargo build script to work around this with `sqlx migrate build-script`.
```

### 5. Set Up .env File
```console
DATABASE_URL=postgres://test_user:test_password@localhost/restaurant_db
```