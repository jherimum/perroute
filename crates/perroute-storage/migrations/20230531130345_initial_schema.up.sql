
CREATE TYPE actor_type AS ENUM ('user', 'system', 'service');
CREATE TYPE dispatch_type AS ENUM ('email', 'sms', 'push');
CREATE TYPE message_status AS ENUM ('pending', 'distributed');
CREATE TYPE message_dispatch_status AS ENUM ('pending', 'queued' ,'success', 'failed');
CREATE TYPE plugin_id AS ENUM ('log', 'smtp', 'sendgrid');

create table connections(
    id          uuid        not null,
    name        varchar     not null,
    plugin_id   plugin_id   not null,
    properties  jsonb       not null,
    enabled     boolean     not null,
    constraint connections_pk primary key (id)
);

CREATE TABLE business_units (
	id 		uuid 	NOT NULL,
	code 	varchar	NOT NULL,
	name 	varchar NULL,
    vars    jsonb   NOT NULL,
	CONSTRAINT business_units_pk 	PRIMARY KEY (id),
	CONSTRAINT business_units_code	UNIQUE (code)
);

create table channels(
    id                  uuid            not null,
    dispatch_type       dispatch_type   not null,
    properties          jsonb           not null,
    enabled             boolean         not null,
    priority            integer         not null,
    connection_id       uuid            not null,
    business_unit_id    uuid            not null, 
    constraint channels_pk      primary key (id),    
    constraint channels_bu_fk   foreign key (business_unit_id) references business_units(id),
    constraint channels_connection_fk foreign key (connection_id) references connections(id)
);

create table message_types(
    id                  uuid            not null,
    code                varchar(50)     not null,    
    name                varchar(500)    not null,
    enabled             boolean         not null,
    vars                jsonb           NOT NULL,
    business_unit_id    uuid            not null,
    CONSTRAINT message_types_pk PRIMARY KEY (id),
    CONSTRAINT message_types_code UNIQUE (code, business_unit_id),
    CONSTRAINT message_types_bu_fk FOREIGN KEY (business_unit_id) REFERENCES business_units(id)
);

create table schemas(
    id                  uuid    NOT NULL,
    value               jsonb   NOT NULL,
    version             integer NOT NULL,
    published           boolean NOT NULL,    
    vars                jsonb   NOT NULL,
    enabled             boolean NOT NULL,
    message_type_id     uuid    NOT NULL,
    business_unit_id    uuid    NOT NULL,

    CONSTRAINT schemas_pk                   PRIMARY KEY (id),
    CONSTRAINT schemas_message_type_number  UNIQUE      (message_type_id, version),
    CONSTRAINT schemas_message_type_fk      FOREIGN KEY (message_type_id)   REFERENCES message_types(id),
    constraint schemas_bu_fk                foreign key (business_unit_id)  references business_units(id)
);

create table templates(
    id                  uuid            not null,
    dispatch_type       dispatch_type   not null,
    subject             text            null,
    text                text            null,
    html                text            null,
    vars                jsonb           NOT NULL,
    active              boolean         not null,
    start_at            timestamp       not null,
    end_at              timestamp       null,
    priority            integer         not null,
    schema_id           uuid            not null,
    message_type_id     uuid            not null,
    business_unit_id    uuid            not null,
    
    constraint templates_pk primary key (id),
    constraint templates_schema_fk          foreign key (schema_id) references schemas(id),
    constraint templates_message_type_fk    foreign key (message_type_id) references message_types(id),
    constraint templates_bu_fk              foreign key (business_unit_id) references business_units(id)
);

create table routes(
    id                  uuid            not null,
    properties          jsonb           not null,
    schema_id           uuid            not null,
    channel_id          uuid            not null,
    business_unit_id    uuid            not null,
    message_type_id     uuid            not null,
    connection_id       uuid           not null,

    constraint routes_pk primary key (id),
    constraint routes_schema_channel unique (schema_id, channel_id),
    constraint routes_schema_fk         foreign key (schema_id)         references schemas(id),
    constraint routes_channel_fk        foreign key (channel_id)        references channels(id),
    constraint routes_bu_fk             foreign key (business_unit_id)  references business_units(id),
    constraint routes_message_type_fk   foreign key (message_type_id)   references message_types(id),
    constraint routes_connection_fk     foreign key (connection_id)     references connections(id)
);


create table messages(
    id                  uuid            not null,
    payload             jsonb           not null,
    deliveries          jsonb           not null,
    status              message_status  not null,
    schema_id           uuid            not null,
    message_type_id     uuid            not null,
    business_unit_id    uuid            not null,
    
    constraint messages_pk primary key (id),
    constraint messages_schema_fk       foreign key (schema_id) references schemas(id),
    constraint messages_message_type_fk foreign key (message_type_id) references message_types(id),
    constraint messages_bu_fk           foreign key (business_unit_id) references business_units(id)
);

create table message_dispatches(
    id                      uuid                    not null,
    message_id              uuid                    not null,
    status                  message_dispatch_status not null,
    result                  jsonb                   not null,
    plugin_id               plugin_id               not null,
    connection_properties   jsonb                   not null,
    dispatcher_properties   jsonb                   not null,
    delivery                jsonb                   not null,

    constraint message_dispatches_pk primary key (id),
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
