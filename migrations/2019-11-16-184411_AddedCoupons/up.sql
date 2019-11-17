-- Your SQL goes here

CREATE TABLE coupons
(
    id serial,
    coupon_code  varchar not null,
    promotion_id integer not null references promotions(id),
    expiration TIMESTAMP with time zone not null,
    primary key (id, promotion_id)
);

CREATE TABLE coupon_uses
(
    coupon_id  integer not null,
    promotion_id integer not null ,
    external_user integer not null,
    PRIMARY KEY (coupon_id,promotion_id, external_user),
    FOREIGN KEY (coupon_id, promotion_id) REFERENCES coupons(id, promotion_id)
)