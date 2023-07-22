CREATE TYPE dispatch_type AS ENUM ('email', 'sms', 'push');

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
    temnplate_id            uuid                null,
    dispatch_type           dispatch_type   not null,
    dispatcher_properties   jsonb           not null,

    channel_id              uuid            not null,
    message_type_id         uuid            not null,
    shcema_id               uuid            not null,
    
    constraint routes_pk primary key (id),
    constraint routes_connection_fk foreign key (connection_id) references connections(id),
    constraint routes_channel_fk foreign key (channel_id) references channels(id),
    constraint routes_message_type_fk foreign key (message_type_id) references message_types(id),
    constraint routes_schema_fk foreign key (schema_id) references schemas(id),
    constraint routes_template_fk foreign key (template_id) references templates(id)
);


CREATE TYPE message_dispatch_status AS ENUM ('pending', 'success', 'failed');


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
