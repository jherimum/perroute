-- Add up migration script here

create table message_type_versions(
    id              uuid    NOT NULL,
    schema          jsonb   NOT NULL,
    number          integer NOT NULL,
    published       boolean NOT NULL,
    message_type_id uuid    NOT NULL,
    channel_id      uuid    NOT NULL,

    CONSTRAINT message_type_versions_pk PRIMARY KEY (id),
    CONSTRAINT message_type_versions_message_type_id_fk FOREIGN KEY (message_type_id) REFERENCES message_types(id),
    CONSTRAINT message_type_versions_channel_id_fk FOREIGN KEY (channel_id) REFERENCES channels(id),
    CONSTRAINT message_type_versions_message_type_id_number UNIQUE (message_type_id, number)
);
