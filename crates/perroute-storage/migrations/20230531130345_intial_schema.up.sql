CREATE TABLE channels (
	id 		uuid 	NOT NULL,
	code 	varchar	NOT NULL,
	name 	varchar NULL,
	CONSTRAINT channels_pk 		PRIMARY KEY (id),
	CONSTRAINT channels_code 	UNIQUE (code)
);

CREATE TABLE passwords (
	id 		uuid NOT NULL,
	user_id uuid NOT NULL,
	hash varchar NOT NULL,
	CONSTRAINT passwords_pk 		PRIMARY KEY(id)
);

CREATE TABLE users (
	id 			uuid 	NOT NULL,
	email 		varchar	NOT NULL,
	password_id uuid 	NOT NULL,
	CONSTRAINT users_pk 			PRIMARY KEY (id),
	CONSTRAINT users_email 			UNIQUE (email),
	CONSTRAINT users_password_fk 	FOREIGN KEY (password_id) REFERENCES passwords(id)
);


CREATE TYPE actor_type AS ENUM ('user', 'system', 'service');

CREATE TABLE command_logs(
	id 				uuid 	NOT NULL,
	command_type	varchar NOT NULL,
	actor_type 		actor_type NOT NULL,
	actor_id 		uuid 	NULL,
	payload 		jsonb 	NOT NULL,
	error 			varchar NULL,
	created_at 		timestamp NOT NULL DEFAULT NOW(),

	CONSTRAINT command_logs_pk 			PRIMARY KEY (id)

)


