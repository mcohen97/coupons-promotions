create table promotions
(
	id integer primary key,
	code varchar not null,
	name varchar not null,
	active boolean not null,
	return_type integer default 0 not null,
	return_value integer not null,
	type varchar not null,
	organization_id integer not null,
	invocations integer default 0 not null,
	negative_responses integer default 0 not null,
	average_response_time double precision default 0.0 not null,
	total_spent double precision default 0.0 not null
);

