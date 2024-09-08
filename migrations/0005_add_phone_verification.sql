-- Add migration script here
ALTER TABLE users
ADD COLUMN phone_verified BOOLEAN DEFAULT FALSE,
ADD COLUMN phone_verification_code TEXT;
