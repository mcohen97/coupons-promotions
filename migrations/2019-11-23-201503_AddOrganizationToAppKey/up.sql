-- Your SQL goes here

ALTER TABLE appkeys ADD organization_id VARCHAR NOT NULL REFERENCES organizations(id);