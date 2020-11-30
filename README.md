# rusty-configs
Manages configs across multiple devices using shared sqlite db file.

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
