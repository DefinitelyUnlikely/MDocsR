# TODO

## Authentication TODO

for authentication, what needs to be done? 
If we start by disregarding anything that would involve endpoints and databases. 

1. We need a way to generate a JWT
2. We need a way to validate the JWT
3. We need a way to decode the JWT to check the sub claim
4. We need a refresh token

All of these things will also require us to be able to get environment variables so that
we can control issuer, audience, expiration time etc etc in a more modular manner, rather than having these things
hardcoded. Hardcoded could be fine for local testing to make sure code works, obviously, but may as well
learn how to configure these things in Rust from the get go.

Once we have these things in place? 

1. Database
2. Database connection
3. Database table for refresh token
4. Login endpoint and corresponding handler/command flow. (Handle token generation, and returning cookies)
5. Logout endpoint to invalidate refresh token
