-- Add migration script here
ALTER TABLE users ADD COLUMN IF NOT EXISTS privacy BOOLEAN DEFAULT FALSE;
