-- name: All :many
SELECT id, name, age, location, email FROM users;

-- name: Add :exec
INSERT INTO users (name, age, location, email)
VALUES (?, ?, ?, ?);

-- name: Edit :exec
UPDATE users SET name = ?, age = ?, location = ?, email = ? WHERE id = ?;

-- name: Delete :exec
DELETE FROM users WHERE id = ?;