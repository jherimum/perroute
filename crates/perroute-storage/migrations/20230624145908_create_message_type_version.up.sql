-- Add up migration script here

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
