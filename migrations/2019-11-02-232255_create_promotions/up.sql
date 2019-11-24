create table organizations
(
	id varchar primary key
);

create table promotions
(
	id serial primary key,
	code varchar not null,
	condition varchar not null,
	name varchar not null,
	active boolean not null,
	return_type varchar not null,
	return_value double precision not null,
	type varchar not null,
	organization_id varchar not null references organizations(id),
	UNIQUE (code, organization_id)
);

