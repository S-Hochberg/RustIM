# RustIM
Rust based instant messaging service.

# TODO LIST:
- [X] Implement base axum based http server
- [X] Implement a database using Postgres and sqlx with a users table
- [X] Implement actual logging
- [ ] Implement user endpoints in the server (register, update)
- [ ] Integrate anyhow
- [ ] Implement http based handshake to create a web socket connection to the client
- [ ] Implement user status (online, offline, away) and last seen
- [ ] Implement basic messaging (text, user to user)
- [ ] Implement chat backlog (user can send X messages and they will get synced to the recipient when they connect)
- [ ] Implement group chating
- [ ] Implement other media types (audio, vido, images, documents)
- [ ] Implement authorization
- [ ] Implement end to end encryption
- [ ] Implement react UI
- [ ] Implement user profile pictures
