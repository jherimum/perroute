-- Add up migration script here

create table schemas(
    id              uuid    NOT NULL,
    schema          jsonb   NOT NULL,
    number          integer NOT NULL,
    published       boolean NOT NULL,
    message_type_id uuid    NOT NULL,

    CONSTRAINT schemas_pk PRIMARY KEY (id),
    CONSTRAINT schemas_message_type_id_fk FOREIGN KEY (message_type_id) REFERENCES message_types(id),
    CONSTRAINT schemas_message_type_id_number UNIQUE (message_type_id, number)
);
