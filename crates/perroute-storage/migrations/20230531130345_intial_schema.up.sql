CREATE TABLE channels (
	id 		uuid 	NOT NULL,
	code 	varchar	NOT NULL,
	name 	varchar NULL,
	CONSTRAINT channels_pk 		PRIMARY KEY (id),
	CONSTRAINT channels_code 	UNIQUE (code)
);

CREATE TABLE users (
	id 			uuid 	NOT NULL,
	email 		varchar	NOT NULL,
	password_id uuid 	NOT NULL,
	CONSTRAINT users_pk 			PRIMARY KEY (id),
	CONSTRAINT users_email 			UNIQUE (email)
);

CREATE TABLE passwords (
	id 		uuid NOT NULL,
	user_id uuid NOT NULL,
	hash varchar NOT NULL,
	CONSTRAINT passwords_pk 		PRIMARY KEY(id),
	CONSTRAINT passwords_user_fk 	FOREIGN KEY(user_id) REFERENCES users(id)
);

ALTER TABLE users ADD CONSTRAINT users_password_fk FOREIGN KEY(password_id) REFERENCES passwords(id);