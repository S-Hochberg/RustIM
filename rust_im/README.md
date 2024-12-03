# RustIM
Rust based instant messaging service.

# TODO LIST:
- [X] Implement base axum based http server
- [X] Implement a database using Postgres and sqlx with a users table
- [X] Implement actual logging (tracing)
- [X] Implement a display macro that just debugs the thing (maybe?)
- [X] Implement create user endpoint with tests
- [X] add a test framework
- [X] Implement get user endpoint
- [ ] Implement http based handshake to create a web socket connection to the client
- [ ] Implement update user endpoint
- [ ] Implement user status (online, offline, away) and last seen
- [ ] Implement basic messaging (text, user to user)
- [ ] Fix all warnings
- [ ] Implement delete user endpoint
- [ ] Implement prettier error json messages
- [ ] Fix env vars to load at runtime
- [ ] Fix sql injection issues (test first)
- [ ] Fix Partial struct to auto derive all attriburtes
- [ ] Change operation structure to work with arc mutexes
- [ ] Add better errors for json parsing issues for requests, sending which fields are missing back
- [ ] Implement chat backlog (user can send X messages and they will get synced to the recipient when they connect)
- [ ] Implement group chating
- [ ] Implement other media types (audio, vido, images, documents)
- [ ] Create a custom error enums pattern - CreateUserErrors
- [ ] Implement authorization
- [ ] Implement end to end encryption
- [ ] Implement react UI
- [ ] Implement user profile pictures
- [ ] Implement auto validator macro on partial types
- [ ] Implement bdd testing framework 
