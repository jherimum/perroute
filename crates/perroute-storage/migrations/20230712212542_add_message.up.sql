CREATE TYPE message_status AS ENUM ('pending', 'distributed');

create table messages(
    id uuid not null,
    payload jsonb not null,
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
