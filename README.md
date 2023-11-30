# Rusty Sword
This project is composed of multiple tools to deal with a korean game about swords.
It is written in rust using ECS and Repository Patterns.

## Sword Server
A server emulator that can be extended with ECS plugins

`$ cargo run -- --mode server`

## Sword Sniffer
A packet sniffer that uses enums to catalog the known packets

`$ cargo run -- --mode sniffer`

## Sword Sandbox
A place to play around and figure out what packets do

`$ cargo run -- --mode sandbox`

## Sword Scripts
A series of scripts used to help developing as an admin

`$ cargo run -- --mode scripts --script create-account admin admin`

## Running migrations
`$ sqlx database create`
`$ sqlx migrate run`

### TODO
- kick player already logged
- backup player stats 
- skill cast sample
- guild load
- inventory load
- skills load
- player statuses

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
- can be aware of components and receive arbitrary scalar arguments but cannot be aware of anything else

### rows
- repositories, only place that can access the database
- should use changesets for updates
- parameters can only receive scalar values

### components
- components that can be used by any plugin including 3rd party plugins
- components that are private to a system should not be in this folder
- should be broken into small components that are the minimum set of information that needs to be accessed on a use case

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