# s12g backend

## Running
need to set `MAILGUN_API_KEY` env variable  
or to disable emailing, set env `DEV=true`

## Misc
Login flow:
1. User enters email
2. Send email with link
3. when link is clicked, create encrypted cookie with email (later to be user id)
test the login flow:  
```
curl -v --cookie cookies --cookie-jar cookies -X POST -H "Content-Type: application/json" -d "{\"email\":\"richodemus@gmail.com\"}" localhost:8000/api/v1/users/login &&
curl -v --cookie cookies --cookie-jar cookies localhost:8000/api/v1/users/login-link?code=fancy-code &&
curl -v --cookie cookies --cookie-jar cookies localhost:8000/api/v1/users/me
```
