
-- ENUM TYPES
CREATE TYPE user_role AS ENUM ('member', 'sponsor', 'admin');
CREATE TYPE application_status AS ENUM ('pending', 'approved', 'rejected');
CREATE TYPE matching_status AS ENUM ('pending', 'accepted', 'declined');
CREATE TYPE report_status AS ENUM ('pending', 'resolved', 'reviewed');
CREATE TYPE reported_type AS ENUM ('message', 'groupchatmessage', 'groupchat', 'user', 'post', 'comment');


-- USERS & AUTHENTICATION
CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE CHECK (char_length(username) >= 3),
    email TEXT NOT NULL UNIQUE CHECK (position('@' IN email) > 1),
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'member',
    banned_until TIMESTAMP NULL,
    avatar_url TEXT NOT NULL CHECK (char_length(avatar_url) > 5),
    created_at TIMESTAMP DEFAULT NOW(),
    dob DATE NOT NULL CHECK (dob <= NOW() - INTERVAL '16 years'),
    user_profile TEXT NOT NULL CHECK (char_length(user_profile) > 10),
    bio TEXT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    email_verification_token UUID NULL,
    forgot_password_token UUID NULL,
    forgot_password_expires_at TIMESTAMP NULL
);


-- SPONSOR APPLICATIONS
CREATE TABLE sponsor_applications (
    application_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    status application_status NOT NULL DEFAULT 'pending',
    application_info TEXT NOT NULL CHECK (char_length(application_info) > 20),
    reviewed_by UUID REFERENCES users(user_id) ON DELETE SET NULL,
    admin_comments TEXT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);


-- MATCHING REQUESTS
CREATE TABLE matching_requests (
    matching_request_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    sponsor_id UUID REFERENCES users(user_id) ON DELETE SET NULL,
    status matching_status NOT NULL DEFAULT 'pending',
    match_score FLOAT CHECK (match_score >= 0 AND match_score <= 100),
    created_at TIMESTAMP DEFAULT NOW()
);


-- MESSAGES & CHATS
CREATE TABLE messages (
    message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    receiver_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL CHECK (char_length(content) > 1),
    timestamp TIMESTAMP DEFAULT NOW(),
    flagged BOOLEAN DEFAULT FALSE,
    deleted BOOLEAN DEFAULT FALSE,
    edited BOOLEAN DEFAULT FALSE,
    seen_at TIMESTAMP NULL
);




-- GROUP CHATS & MEMBERS
CREATE TABLE group_chats (
    group_chat_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP DEFAULT NOW(),
    flagged BOOLEAN DEFAULT FALSE
);

CREATE TABLE group_chat_members (
    group_chat_id UUID REFERENCES group_chats(group_chat_id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    PRIMARY KEY (group_chat_id, user_id)
);
CREATE TABLE group_chat_messages (
    group_chat_message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_chat_id UUID REFERENCES group_chats(group_chat_id) ON DELETE CASCADE,
    sender_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL CHECK (char_length(content) > 1),
    timestamp TIMESTAMP DEFAULT NOW(),
    flagged BOOLEAN DEFAULT FALSE,
    deleted BOOLEAN DEFAULT FALSE,
    edited BOOLEAN DEFAULT FALSE
);

-- GROUP MEETINGS & PARTICIPANTS
CREATE TABLE group_meetings (
    meeting_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_chat_id UUID REFERENCES group_chats(group_chat_id) ON DELETE SET NULL,
    host_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    title TEXT NOT NULL CHECK (char_length(title) >= 5),
    description TEXT NULL,
    scheduled_time TIMESTAMP NOT NULL
);

CREATE TABLE meeting_participants (
    meeting_id UUID REFERENCES group_meetings(meeting_id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    PRIMARY KEY (meeting_id, user_id)
);


-- RESOURCE LIBRARY
CREATE TABLE resources (
    resource_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contributor_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    title TEXT NOT NULL CHECK (char_length(title) >= 5),
    content TEXT NOT NULL CHECK (char_length(content) > 20),
    approved BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);


-- REPORTS
CREATE TABLE reports (
    report_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reporter_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    reported_user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    reason TEXT NOT NULL CHECK (char_length(reason) >= 10),
    reported_type reported_type NOT NULL,
    reported_item_id UUID NOT NULL,
    status report_status NOT NULL DEFAULT 'pending',
    reviewed_by UUID REFERENCES users(user_id) ON DELETE SET NULL,
    resolved_at TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT NOW()
);


-- POSTS & COMMENTS
CREATE TABLE posts (
    post_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL CHECK (char_length(content) > 5),
    flagged BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE post_likes (
    post_id UUID REFERENCES posts(post_id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, user_id)
);

CREATE TABLE comments (
    comment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID REFERENCES posts(post_id) ON DELETE CASCADE,
    author_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
    parent_comment_id UUID REFERENCES comments(comment_id) ON DELETE SET NULL,
    content TEXT NOT NULL CHECK (char_length(content) > 1),
    created_at TIMESTAMP DEFAULT NOW()
);
