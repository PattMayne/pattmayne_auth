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
 * Switch to PostgreSQL because bools are bools instead of ints (but maybe that's not important)
 * move env variable from .env to somewhere more secure for production
 * Make Error page take Http code args and maybe a message
 * Remove magic strings from front-end JS. Put them in globals or resources file.
 * Make tests, especially for error page.
 * Table in DB to store accepted sites (client sites)
 * Admin page for adding/removing client sites

### Client Tokens Structure:
The client apps will set JWTs as access tokens into the user's browser's secure cookies. JWTs will expire every few minutes (somewhere within an hour) and be refreshed based on user's refresh token (which is also stored in a secure cookie). JWTs are not stored on any server, only in the browser. But each client app can verify the token, and each client app has its own JWT secret.

The auth app (this app) will issue the refresh tokens and save them in the database. **Problem:** the auth app cannot set cookies for a user who is interacting with a different URL. **Solution:** when the user's refresh_token expires, the client app will use its client_secret to communicate with the auth app (this app), and the auth app will issue a new refresh_token. The client app can then set the refresh token (and a new JWT) into the user's browser's secure cookies.

Client sites are stored in a clients table in the DB.
Refresh tokens are stored in a refresh_tokens table in the DB. Refresh token entries include a user_id and a client_id. A user has a different token for each client. The expiry date of each token should be the same, to ensure that the user is made to log in periodically. When the user is logged into one client, they are logged into all. But when one refresh token expires, they all expire.

For most requests the user makes on the client site, they do NOT need to interact with the auth app (this app). Client apps have some autonomy.