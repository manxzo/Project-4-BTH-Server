-- Add migration script here

-- ENUM TYPES (Ensure they exist before creating new ones)
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('member', 'sponsor', 'admin');
EXCEPTION WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE application_status AS ENUM ('pending', 'approved', 'rejected');
EXCEPTION WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE matching_status AS ENUM ('pending', 'accepted', 'declined');
EXCEPTION WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE report_status AS ENUM ('pending', 'resolved', 'reviewed');
EXCEPTION WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE reported_type AS ENUM ('message', 'groupchatmessage', 'groupchat', 'user', 'post', 'comment');
EXCEPTION WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE announcement_type AS ENUM ('general', 'recommended', 'meetingreminder', 'invitation');
EXCEPTION WHEN duplicate_object THEN null;
END $$;


-- USERS TABLE (Ensure it exists, modify only necessary fields)
ALTER TABLE users ADD COLUMN IF NOT EXISTS banned_until TIMESTAMP NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verified BOOLEAN DEFAULT FALSE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verification_token UUID NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS forgot_password_token UUID NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS forgot_password_expires_at TIMESTAMP NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS location JSONB NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS interests JSONB NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS experience JSONB NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS available_days JSONB NULL;
ALTER TABLE users ADD COLUMN IF NOT EXISTS languages JSONB NULL;

-- SPONSOR APPLICATIONS TABLE
ALTER TABLE sponsor_applications ADD COLUMN IF NOT EXISTS admin_comments TEXT NULL;


-- MATCHING REQUESTS TABLE (Ensure match_score exists)
ALTER TABLE matching_requests ADD COLUMN IF NOT EXISTS match_score FLOAT CHECK (match_score >= 0 AND match_score <= 100);


-- MESSAGES TABLE (Ensure seen_at exists)
ALTER TABLE messages ADD COLUMN IF NOT EXISTS seen_at TIMESTAMP NULL;


-- GROUP CHATS TABLE (Ensure flagged column exists)
ALTER TABLE group_chats ADD COLUMN IF NOT EXISTS flagged BOOLEAN DEFAULT FALSE;


-- GROUP MEETINGS TABLE (Ensure new fields exist)
ALTER TABLE group_meetings ADD COLUMN IF NOT EXISTS description TEXT NULL;
ALTER TABLE group_meetings ADD COLUMN IF NOT EXISTS scheduled_time TIMESTAMP NOT NULL;

CREATE TABLE IF NOT EXISTS matching_requests (
    matching_request_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    sponsor_id UUID REFERENCES users(user_id) ON DELETE SET NULL,
    status matching_status NOT NULL DEFAULT 'pending',
    match_score FLOAT CHECK (match_score >= 0 AND match_score <= 100),
    created_at TIMESTAMP DEFAULT NOW()
);


-- ANNOUNCEMENTS TABLE (New Table)
CREATE TABLE IF NOT EXISTS announcements (
    announcement_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    announcement_type announcement_type NOT NULL,
    announcement_target_id UUID NULL, 
    message TEXT NOT NULL CHECK (char_length(message) > 5),
    created_at TIMESTAMP DEFAULT NOW()
);

