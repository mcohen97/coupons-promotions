-- Your SQL goes here

CREATE TABLE appkeys(
    promotion_id INTEGER NOT NULL REFERENCES promotions(id),
    token varchar(21) NOT NULL,
    PRIMARY KEY (promotion_id, token)
)