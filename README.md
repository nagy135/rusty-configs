# rusty-configs

![](https://tokei.rs/b1/github/nagy135/rusty-configs?category=code)

Manages configs across multiple devices using shared sqlite db file.
Syncs configs to a sqlite one-file-database, holding multiple versions
with option to update system's files (as well as update database versions).

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
* list versions
* update version
* minimize duplicity in entities.rs
