# PATTMAYNE AUTH

An authentication app I will use to serve a few web apps and games I intend to make.
I'll use JSON webtokens (JWTs) reinforced by refresh_tokens.

### TO DO:
 * create resources file (probably just a constants file)
 * suspend IP address if too many failed attempts
 * create endpoints for another app to authenticate
 * * new routes file called external_routes
 * * create an enum of apps that can use this
 * * or MAYBE they should be in the DB instead.
 * Containerize with Docker
 * move env variable from .env to somewhere more secure for production
 * Remove magic strings from front-end JS. Put them in globals or resources file.
 * * Use yaml files (flat style)
 * * NO: use use phf::phf_map;  ( MUCH faster, though more verbose )
 * Make tests
 * When a person tries to register on a site, but they're already registered on another, just log them in
 * * This will create a refresh token for THAT site
 * * I don't think they can auto-log in to ALL sites. They must log in to get the JWT cookie.
 * * So each site will have its own expiry for refresh tokens.
 * * BUT logging out of one will log out of all.
 * * Maybe logging IN to one should extend the expiry of all? But only those that already exist.
 * Email verification of accounts (with Mailjet https://www.mailjet.com/pricing/)
 * Integrate payment APIs (probably with reqwest crate)
 * * Integrate Stripe for payments
 * * Accept crypto currency with a 3rd party API (like coinbase)

### Client Tokens Structure:
The client apps will set JWTs as access tokens into the user's browser's secure cookies. JWTs will expire every few minutes (somewhere within an hour) and be refreshed based on user's refresh token (which is also stored in a secure cookie). JWTs are not stored on any server, only in the browser. But each client app can verify the token, and each client app has its own JWT secret.

The auth app (this app) will issue the refresh tokens and save them in the database. **Problem:** the auth app cannot set cookies for a user who is interacting with a different URL. **Solution:** when the user's refresh_token expires, the client app will use its client_secret to communicate with the auth app (this app), and the auth app will issue a new refresh_token. The client app can then set the refresh token (and a new JWT) into the user's browser's secure cookies.

Client sites are stored in a clients table in the DB.
Refresh tokens are stored in a refresh_tokens table in the DB. Refresh token entries include a user_id and a client_id. A user has a different token for each client. The expiry date of each token should be the same, to ensure that the user is made to log in periodically. When the user is logged into one client, they are logged into all. But when one refresh token expires, they all expire.

For most requests the user makes on the client site, they do NOT need to interact with the auth app (this app). Client apps have some autonomy.
