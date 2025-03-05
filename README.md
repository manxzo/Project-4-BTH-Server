# BTH: Beyond The Horizon - Project Proposal

## Introduction

**BTH: Beyond The Horizon** is a peer-support platform designed to provide structured and meaningful connections between individuals seeking support and volunteers willing to help. Unlike traditional therapy-based platforms, BTH allows members to connect with sponsors—volunteers who offer guidance, accountability, and community support. This platform aims to create an accessible, anonymous, and secure space for individuals to find encouragement and practical help from those who have walked similar paths.

## Utility of This App

BTH serves as a bridge between people looking for emotional and mental health support and volunteers willing to guide them. Many individuals face challenges but lack access to professional therapy due to financial, social, or logistical reasons. BTH provides an alternative by:

- Offering **one-on-one peer support** in a structured and anonymous environment.
- Enabling **real-time messaging** to ensure timely guidance and encouragement.
- Facilitating **group discussions and meetings** to foster a supportive community.
- Providing **a library of resources** curated by both sponsors and community members.
- Ensuring **user safety** through authentication, content moderation, and privacy measures.

## Tech Stack

- **Frontend:** Next.js (React) + Hero UI for UI components
- **Backend:** Rust + Axum (API server)
- **Database:** PostgreSQL (SQLx for Rust ORM)
- **Authentication:** JWT-based authentication (stored in HTTP-only cookies)
- **Messaging:** WebSockets for real-time communication

## Features & Functionality

### 1. **User Authentication**

- Users can sign up as either **Members** (seeking support) or **Sponsors** (providing support).
- JWT-based authentication with secure password hashing.
- Protected routes to ensure privacy.

### 2. **User Profiles & Matching**

- Members can browse and request sponsors.
- Sponsors can accept or decline support requests.
- Matching algorithm for best-fit sponsor recommendations.

### 3. **Messaging & Real-Time Chat**

- Secure WebSocket-based messaging system.
- Private chat between matched users.
- Group chat for community discussions.

### 4. **Group Meetings & Scheduling**

- Users can join or create support meetings.
- Calendar integration for scheduling sessions.
- Automated reminders for upcoming meetings.

### 5. **Resource Library**

- Collection of guides, exercises, and articles.
- Users can contribute and share resources.
- Moderation system for reviewing content.

### 6. **Admin Dashboard**

- Manage users, reports, and flagged content.
- Monitor system performance and logs.

## Development Workflow

### **Week 1: Backend Development**

1. **Setup & Configuration**

   - Install Rust, Axum, PostgreSQL
   - Set up database schema

2. **Authentication & User Management**

   - JWT-based authentication
   - Implement user registration & login

3. **Sponsor-Member Matching**

   - Design algorithms for optimal matching

4. **Messaging System**

   - Implement WebSockets for real-time chat

### **Week 2: Frontend Development**

5. **Next.js UI & Routing**

   - Implement Hero UI components
   - Create authentication pages

6. **Integrate API with Next.js**

   - Fetch & display user data
   - Connect frontend with Axum API

7. **Messaging & Meetings UI**

   - Develop chat interface
   - Implement calendar for scheduling

### **Week 3: Optimization & Deployment**

8. **Optimize Database Queries**

   - Use indexing & caching for performance

9. **Security & Testing**

   - Secure API endpoints
   - Implement unit & integration tests

10. **Deployment**

    - Deploy Axum API & PostgreSQL database
    - Deploy Next.js frontend

## Pseudocode & System Flow

```plaintext
User Registration:
  IF new user registers
    Validate input
    Hash password
    Store in database
    Return JWT token

User Authentication:
  IF user logs in
    Verify credentials
    Generate JWT
    Return user session data

Sponsor-Member Matching:
  IF user requests a sponsor
    Find best match from available sponsors
    Store connection in database

Messaging System:
  IF message sent
    Store message in database
    Broadcast via WebSocket
    Deliver in real-time to recipient
```

## Deployment Strategy

- **Backend (Axum API + PostgreSQL)** deployed on **Fly.io or DigitalOcean**.
- **Frontend (Next.js)** hosted on **Vercel**.
- **Database managed on a PostgreSQL cloud service**.

## Conclusion

BTH: Beyond The Horizon aims to provide a structured, scalable, and safe environment for individuals to seek and offer peer-based support. By integrating real-time communication, structured sponsor-matching, and community-driven resources, the platform ensures a meaningful and effective experience for all users.

---

**Next Steps:**

- Implement PostgreSQL schema
- Set up authentication system

🚀 **Let’s start building!**

