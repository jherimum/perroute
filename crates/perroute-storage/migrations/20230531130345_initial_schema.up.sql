CREATE TABLE channels (
	id 		uuid 	NOT NULL,
	code 	varchar	NOT NULL,
	name 	varchar NULL,
	CONSTRAINT channels_pk 		PRIMARY KEY (id),
	CONSTRAINT channels_code	UNIQUE (code)
);

CREATE TABLE passwords (
	id 		uuid NOT NULL,
	user_id uuid NOT NULL,
	hash varchar NOT NULL,
	CONSTRAINT passwords_pk	PRIMARY KEY(id)
);

CREATE TABLE users (
	id 			uuid 	NOT NULL,
	email 		varchar	NOT NULL,
	password_id uuid 	NOT NULL,
	CONSTRAINT users_pk 			PRIMARY KEY (id),
	CONSTRAINT users_email 			UNIQUE (email),
	CONSTRAINT users_password_fk	FOREIGN KEY (password_id) REFERENCES passwords(id)
);


CREATE TYPE actor_type AS ENUM ('user', 'system', 'service');

CREATE TABLE command_logs(
	id 				uuid 		NOT NULL,
	command_type	varchar 	NOT NULL,
	actor_type 		actor_type	NOT NULL,
	actor_id 		uuid 		NULL,
	payload 		jsonb 		NOT NULL,
	error 			varchar 	NULL,
	created_at 		timestamp 	NOT NULL DEFAULT NOW(),

	CONSTRAINT command_logs_pk	PRIMARY KEY (id)

);


create table message_types(
    id          uuid            not null,
    code        varchar(50)     not null,    
    description varchar(500)    not null,
    enabled     boolean         not null,
    channel_id  uuid            not null,    
    CONSTRAINT message_types_pk PRIMARY KEY (id),
    CONSTRAINT message_types_code UNIQUE (code),
    CONSTRAINT message_types_channel_fk FOREIGN KEY (channel_id) REFERENCES channels(id)
);

create table schemas(
    id              uuid    NOT NULL,
    schema          jsonb   NOT NULL,
    version         integer NOT NULL,
    published       boolean NOT NULL,
    message_type_id uuid    NOT NULL,
    channel_id      uuid    NOT NULL,

    CONSTRAINT schemas_pk PRIMARY KEY (id),
    CONSTRAINT schemas_message_type_fk FOREIGN KEY (message_type_id) REFERENCES message_types(id),
    CONSTRAINT schemas_message_type_number UNIQUE (message_type_id, version),
    CONSTRAINT schemas_channel_fk FOREIGN KEY (channel_id) REFERENCES channels(id)
);

create table templates(
    id              uuid            not null,
    code            varchar(255)    not null,
    description     varchar(100)    null,
    template        text            not null,
    schema_id       uuid            not null,

    constraint templates_pk primary key (id),
    constraint templates_schema_fk foreign key (channel_id) references schemas(id),
    constraint templates_code_channel_un unique (code, channel_id)
);
