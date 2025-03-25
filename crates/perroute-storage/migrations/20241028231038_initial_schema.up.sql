CREATE TYPE message_status AS ENUM ('pending', 'dispatched', 'failed');

create table business_units (
    id              varchar(21) primary key,
    code            varchar(50) not null,
    name            text not null,
    vars            jsonb null,
    created_at      timestamp not null,
    updated_at      timestamp not null,
    constraint business_unit_code_idx unique (code)
);

create table channels (
    id                  varchar(21) primary key,
    name                text not null,
    business_unit_id    varchar(21) not null,
    dispatch_type       varchar(50) not null,
    provider_id         varchar not null,
    configuration       jsonb not null,
    enabled             boolean not null,
    created_at          timestamp not null,
    updated_at     timestamp not null,
    constraint channel_bu_fk foreign key (business_unit_id) references business_units(id)
);


create table message_types(
    id                  varchar(21) primary key,
    name                varchar(500) not null,
    code                varchar not null,
    schema              jsonb not null,
    vars                jsonb null,
    enabled             boolean not null,
    created_at          timestamp not null,
    updated_at          timestamp not null,
    constraint message_types_code_unique unique(code)
);

create table payload_examples(
    id                  varchar(21) primary key,
    message_type_id     varchar(21) not null,
    name                varchar(500) not null,
    payload             jsonb not null,
    constraint fk_payload_axamples_message_types foreign key (message_type_id) references message_types(id)
);


create table routes(
    id                  varchar(21) primary key,
    name                varchar(500) not null,
    configuration       jsonb not null,
    priority            integer not null,
    enabled             boolean not null,
    channel_id          varchar(21) not null,
    message_type_id     varchar(21) not null,
    created_at          timestamp not null,
    updated_at          timestamp not null,
    constraint fk_routes_channels      foreign key (channel_id) references channels(id),
    constraint fk_routes_message_types foreign key (message_type_id) references message_types(id)
);

create table template_assignments (
    id                  varchar(21) primary key,
    message_type_id     varchar(21) not null,
    business_unit_id    varchar(21) not null,
    vars                jsonb null,
    priority            int not null,
    start_at            timestamp not null,
    end_at              timestamp null,
    enabled             boolean not null,
    dispatch_type       varchar(50) not null,    
    created_at          timestamp not null,
    updated_at          timestamp not null,
    constraint template_assignment_message_type_fk foreign key (message_type_id) references message_types(id),
    constraint template_assignment_business_unit_fk foreign key (business_unit_id) references business_units(id)  
);

create table messages(
    id                  varchar(21) primary key,
    key                 varchar(50) null,
    message_type_id     varchar(21) not null,
    business_unit_id    varchar(21) not null,
    payload             jsonb not null,
    recipient           jsonb not null,
    dispatch_type       varchar(50) not null,
    status              message_status not null,
    tags                jsonb not null,
    scheduled_at        timestamp null,
    created_at          timestamp not null,
    updated_at          timestamp not null,
    constraint fk_messages_message_types foreign key (message_type_id) references message_types(id),
    constraint fk_messages_business_units foreign key (business_unit_id) references business_units(id)    
);


create table message_dispatches(
    id              varchar(21) primary key,
    message_id      varchar(21) not null,
    provider_id     varchar(21) not null,
    success         boolean not null,
    created_at      timestamp not null,
    updated_at timestamp not null,
    constraint fk_message_dispatches_messages foreign key (message_id) references messages(id)
);


create table event_messages(
    id              varchar(21) primary key,
    event_type      varchar(100) not null,
    entity_id       varchar(21) not null,
    payload         jsonb not null,
    actor_type      varchar(100) not null,
    actor_id        varchar(21)  null,
    created_at      timestamp not null,    
    consumed_at     timestamp null,
    skipped         boolean null
);
