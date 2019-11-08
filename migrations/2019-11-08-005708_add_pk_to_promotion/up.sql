-- Your SQL goes here

ALTER TABLE promotions ADD organization_id integer NOT NULL REFERENCES organizations(id);