-- Add migration script here
ALTER TABLE users
ADD COLUMN email_verified BOOLEAN DEFAULT FALSE, -- To track if the email is verified
ADD COLUMN verification_token UUID DEFAULT gen_random_uuid(); -- A unique token for email verification
