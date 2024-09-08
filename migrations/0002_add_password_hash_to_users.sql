-- Drop existing users table if it exists (optional, for development purposes)
DROP TABLE IF EXISTS users CASCADE;

-- Re-create the users table with password_hash
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,  -- Add this field to store hashed passwords
    phone_number TEXT,  -- Optional field for phone numbers
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
