# Rusty Sword

## How to Run
`$ cargo run -- --mode server`
`$ cargo run -- --mode sniffer`
`$ cargo run -- --mode sandbox`

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