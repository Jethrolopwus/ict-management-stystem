## Running the app
~~~
cargo run
~~~

### To Connect to DB
~~~
sudo -u postgres psql -d ict_management
~~~
## To Run migration 
terminal run:
~~~
diesel migration run
~~~


## Endpoints.

You have the following endpoints running on http://127.0.0.1:8080:
Create user: POST /users
Update user: PUT /users/{id}
Deactivate user: PUT /users/{id}/deactivate

### Create User

~~~
{
    "username": "admin1",
    "email": "admin1@example.com",
    "password_hash": "plaintextpassword",
    "role": "Admin"
  }
~~~

### Update User
~~~
 {
    "username": "admin1_updated",
    "email": "admin1_updated@example.com"
  }
~~~
  ### Deactivate User
Replace <USER_ID> with the actual UUID.
 PUT http://127.0.0.1:8080/users/<USER_ID>/deactivate
