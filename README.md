# rusty-configs
Manages configs across multiple devices using shared sqlite db file.

# seriousness
This project doesnt try to be anything serious, its just learning project
trying to build some codebase around rust's sqlite binding.
During implementation I m trying to build some "entity abstraction", defining
traits that avoid replicating code on multiple structs.

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
