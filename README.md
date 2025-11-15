# PATTMAYNE AUTH

An authentication app I will use to serve a few web apps and games I intend to make.
I'll use JSON webtokens and a database.

### TO DO:
 * create resources file (probably just a constants file)
 * nav in header included in template ( HOME | LOGOUT | LOGIN | REGISTER ) (send User obj to header template)
 * suspend IP address if too many failed attempts
 * create endpoints for another app to authenticate
 * * new routes file called external_routes
 * * create an enum of apps that can use this
 * * or MAYBE they should be in the DB instead.
 * Containerize with Docker
 * Switch to PostgreSQL because bools are bools instead of ints (but maybe that's not important)
 * move env variable from .env to somewhere more secure for production
 * Checking that user is logged in (UserReqData has id) uses cumbersome nested pattern matching
 * * update this into some kind of function, or at least simpler code
 * * match { Ok() => None should always return to login (repeated code)
 * * Err should be able to get some Error code (?) and send to the Error page
 * Test Error page
 * Make Error page take Http code args and maybe a message
 * Remove magic strings from front-end JS. Put them in globals or resources file.