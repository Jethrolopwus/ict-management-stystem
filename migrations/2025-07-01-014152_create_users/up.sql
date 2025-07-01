   CREATE EXTENSION IF NOT EXISTS "pgcrypto";

   CREATE TABLE users (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       username VARCHAR NOT NULL UNIQUE,
       email VARCHAR NOT NULL UNIQUE,
       password_hash VARCHAR NOT NULL,
       role VARCHAR NOT NULL,
       active BOOLEAN NOT NULL DEFAULT TRUE,
       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP NOT NULL DEFAULT NOW()
   );

   
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
