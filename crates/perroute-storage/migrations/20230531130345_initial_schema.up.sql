
CREATE TYPE actor_type AS ENUM ('user', 'system', 'service');
CREATE TYPE dispatch_type AS ENUM ('email', 'sms', 'push');
CREATE TYPE message_status AS ENUM ('pending', 'distributed');
CREATE TYPE message_dispatch_status AS ENUM ('pending', 'queued' ,'success', 'failed');


CREATE TABLE channels (
	id 		uuid 	NOT NULL,
	code 	varchar	NOT NULL,
	name 	varchar NULL,
    enabled boolean NOT NULL,
    vars    jsonb   NOT NULL,
	CONSTRAINT channels_pk 		PRIMARY KEY (id),
	CONSTRAINT channels_code	UNIQUE (code)
);

create table message_types(
    id          uuid            not null,
    code        varchar(50)     not null,    
    name        varchar(500)    not null,
    enabled     boolean         not null,
    vars        jsonb           NOT NULL,
    CONSTRAINT message_types_pk PRIMARY KEY (id),
    CONSTRAINT message_types_code UNIQUE (code)
);

create table schemas(
    id              uuid    NOT NULL,
    schema          jsonb   NOT NULL,
    version         integer NOT NULL,
    published       boolean NOT NULL,    
    message_type_id uuid    NOT NULL,
    vars    jsonb   NOT NULL,
    enabled         boolean NOT NULL,

    CONSTRAINT schemas_pk                   PRIMARY KEY (id),
    CONSTRAINT schemas_message_type_fk      FOREIGN KEY (message_type_id)   REFERENCES message_types(id),
    CONSTRAINT schemas_message_type_number  UNIQUE      (message_type_id, version)
    
);


create table templates(
    id              uuid            not null,
    name            varchar(255)    not null,
    subject         text            null,
    text            text            null,
    html            text            null,
    channel_id      uuid            not null,
    message_type_id uuid            not null,
    vars            jsonb           NOT NULL,
    dispatch_type   dispatch_type   not null,

    constraint templates_pk primary key (id),
    CONSTRAINT templates_channel_fk         FOREIGN KEY (channel_id)        REFERENCES channels(id),
    CONSTRAINT templates_message_type_fk    FOREIGN KEY (message_type_id)   REFERENCES message_types(id)
);

create table template_compabilities(
    template_id     uuid            not null,
    schema_id       uuid            not null,

    constraint template_compabilities_pk            primary key (template_id, schema_id),
    CONSTRAINT template_compabilities_template_fk   FOREIGN KEY (template_id)   REFERENCES templates(id),
    CONSTRAINT template_compabilities_schema_fk     FOREIGN KEY (schema_id)     REFERENCES schemas(id)
);

create table connections(
    id          uuid    not null,
    name        varchar not null,
    plugin_id   varchar not null,
    properties  jsonb   not null,
    enabled     boolean not null,
    constraint connections_pk primary key (id)
);

create table routes(
    id                      uuid            not null,
    name                    varchar         not null,
    connection_id           uuid            not null,
    template_id             uuid                null,
    dispatch_type           dispatch_type   not null,
    dispatcher_properties   jsonb           not null,

    channel_id              uuid            not null,
    message_type_id         uuid            not null,
    schema_id               uuid            not null,
    
    constraint routes_pk primary key (id),
    constraint routes_connection_fk foreign key (connection_id) references connections(id),
    constraint routes_channel_fk foreign key (channel_id) references channels(id),
    constraint routes_message_type_fk foreign key (message_type_id) references message_types(id),
    constraint routes_schema_fk foreign key (schema_id) references schemas(id),
    constraint routes_template_fk foreign key (template_id) references templates(id)
);




create table messages(
    id uuid not null,
    payload jsonb not null,
    recipient jsonb not null,
    exclude_dispatcher_types jsonb not null,
    include_dispatcher_types jsonb not null,
    status message_status not null,
    scheduled_to timestamp null,
    schema_id uuid not null,
    message_type_id uuid not null,
    channel_id uuid not null,

    CONSTRAINT messages_pk PRIMARY KEY (id),
    CONSTRAINT messages_schema_fk FOREIGN KEY (schema_id) REFERENCES schemas(id),
    constraint messages_message_type_fk FOREIGN KEY (message_type_id) REFERENCES message_types(id),
    constraint messages_channel_fk FOREIGN KEY (channel_id) REFERENCES channels(id)
);

create table message_dispatches(
    id              uuid                    not null,
    message_id      uuid                    not null,
    route_id        uuid                    not null,    
    status          message_dispatch_status not null,
    result          jsonb                   not null,

    constraint message_dispatches_pk primary key (id),
    constraint message_dispatches_route_fk foreign key (route_id) references routes(id),
    constraint message_dispatches_message_fk foreign key (message_id) references messages(id)
);


create table events(   
    id              uuid                    not null,
    entity_id       uuid                    not null,
    event_type      text                    not null,    
    created_at      timestamp               not null,
    scheduled_to    timestamp               not null,
    consumed_at     timestamp               null,
    constraint events_pk primary key (id)
);

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
