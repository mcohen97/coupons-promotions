create table promotions
(
	id serial primary key,
	code varchar not null,
	name varchar not null,
	active boolean not null,
	return_type varchar not null,
	return_value double precision not null,
	type varchar not null,
	organization_id integer not null
);

