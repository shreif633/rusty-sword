CREATE TABLE IF NOT EXISTS items
(
    id INTEGER PRIMARY KEY NOT NULL,
    player_id INTEGER NOT NULL,
    item_index SMALLINT NOT NULL,
    prefix TINYINT NOT NULL,
    quantity INTEGER NOT NULL,
    maximum_endurance TINYINT NOT NULL,
    current_endurance TINYINT NOT NULL,
    physical_attack_talisman TINYINT NOT NULL,
    magical_attack_talisman TINYINT NOT NULL,
    talisman_of_accuracy TINYINT NOT NULL,
    talisman_of_defence TINYINT NOT NULL,
    upgrade_level TINYINT NOT NULL,
    upgrade_rate TINYINT NOT NULL
);