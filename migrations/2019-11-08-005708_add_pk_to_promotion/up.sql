-- Your SQL goes here

ALTER TABLE promotions ADD organization_id varchar NOT NULL REFERENCES organizations(id);