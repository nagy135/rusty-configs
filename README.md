# rusty-configs

![](https://tokei.rs/b1/github/nagy135/rusty-configs?category=code)

Manages configs across multiple devices using shared sqlite db file.
Syncs configs to a sqlite one-file-database, holding multiple versions
with option to update system's files (as well as update database versions).
Encodes data with base64 to database to retain all the symbols after file recovery.

# dependecies
* rustc
* cargo

# installation
in development still ....but
```
git clone https://github.com/nagy135/rusty-configs
cd rusty-configs
cargo build
sudo cp target/debug/rusty-configs /usr/local/bin
```
# usage
first we initialize db (creating db file and initializing tables)
```
rusty-configs init
```

now we need to create version, representing different workspaces
```
rusty-configs add -v home
```

now we can store configs with this version
```
rusty-configs add -p /path/to/file -v home
```

File is now stored in sqlite db and can be sent to a different device of yours.
You install rusty-configs on that one as well placing sqlite.db file in the same location and 

```
rusty-configs write
```

All the files are created (run with sudo if needed)

Configs can be listed
```
rusty-configs list configs
```

So can versions
```
rusty-configs list versions
```

We can refresh data in these stored configs 
```
rusty-configs read
```

To remove config from db
```
rusty-configs delete config -p /path/to/file
```

To remove version from db
```
rusty-configs delete version -v home
```
# seriousness
This project doesnt try to be anything serious, its just learning project
trying to build some codebase around rust's sqlite binding.
It honestly doesnt make much sense to store files in database, since folder would do.
Idea is to have this simple portable sqlite file that holds all the versions and
will help me to sync configs across devices.
During implementation I m trying to build some "entity abstraction", defining
traits that avoid replicating code on multiple structs, trying to get as close
as possible to writing zero SQL after "entity abstraction" is done. You can think
of it as a trivial ORM.

# features
* holds global versions (example: 'work', 'home')
* has commands to store and retrieve configs to/from their locations

# db structure
Uses sqlite database with trivial model structure

## version
### name
* string
* contains name of version

## config
### path
* string
* path to location of config
### data
* text
* lines of config
### version
* private key to version

# dependecies
* rust

# TODO
* read parameter to refresh only specific version
* missing prints after running command
* install process
