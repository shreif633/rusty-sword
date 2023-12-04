# Rusty Sword
This project is composed of multiple tools to deal with a korean game about swords.
If you mention the game, you lose the game.
It is written in rust using ECS and Repository Patterns.
This is rapidly changing and is still experimental.

## Sword Server
A server emulator that can be extended with ECS plugins

`$ cargo run -- --mode server`

## Sword Sniffer
A packet sniffer that uses enums to catalog the known packets

`$ cargo run -- --mode sniffer`

hint: use the following command to output the sniffer results to a file

`$ cargo run -- --mode sniffer > file_name.txt`

## Sword Scripts
A series of scripts used to help developing as an admin

`$ cargo run -- --mode scripts --script create-account admin admin`

## Setting up the project
`$ git clone (url)`
`$ cd rusty-sword`
`$ sqlx database create`
`$ sqlx migrate run`
`$ cargo run -- --mode scripts`

## Project Structure

### requests
- packets coming from the clients
- each packet becomes a tag
- tags are added before update and removed after update
- must derive Component
- do not know about the rest of the world

### responses
- packets sent from the server
- must implement into and from packet
- must implement new
- can be aware of components and bundle and receive arbitrary scalar arguments but cannot be aware of anything else
- the exception are login related responses that can be aware of rows

### rows
- repositories, only place that can access the database
- should use changesets for updates
- parameters can only receive scalar values

### components
- components that can be used by any plugin including 3rd party plugins
- components that are private to a system should not be in this folder
- should be broken into small components that are the minimum set of information that needs to be accessed on a use case
- components can be aware of configs and rows

### bundles
- prefab that groups components, follows same rules as components

### framework
- do not touch this folder

### db
- auto generated folder 
- this is meant for development mode only 
- do not use sqlite in production

### migrations
- sqlx migrations
- always use migrations to change the database
- do not be afraid of having as many migrations as you need

### repositories
- this is the only place that should access the database directly
- use the repository pattern
- follows naming conventions based on exlixir phoenix 
- use entire rows to read
- use small changesets to write

## ECS Stages
- first: read packets from clients
- pre update: validate data, like checking if a skill is in cooldown
- update: run your custom scripts here
- post update: use this to bypass systems, like removing damage after its dealt but before its accounted
- last: send packets to clients based on the updates that made throught the frame

## How to contribute
- The master branch is only for stable and reviewed code
- Fork this repo and checkout the `development` branch
- Make your changes
- Submit a PR agains the `development` Branch

### Naming Conventions
- postfix structs with the singular form of its folder: ex `plugins/health.rs` -> `HealthPlugin`
- components should not have a postfix
- fields should have explicit long names and should never have abbreviations
- if you do not know what a packet field does name it "unknown" so we can quickly lookup for things to investigate
- if you do not know two or more items name it "unknown1" and "unknown2" and so on
- fields that have vunevaribilities should be prefixed with "unsafe_"
- fields related to entity index or database id should be postfixed with "_id"
- fields related to config index should be postfixed with "_index"