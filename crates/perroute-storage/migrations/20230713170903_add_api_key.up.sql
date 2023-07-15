create table api_keys (
    id          uuid        not null,    
    name        varchar     not null,
    prefix      varchar     not null,
    hash        varchar     not null,
    created_at  timestamp   not null,
    revoked_at  timestamp,
    expires_at  timestamp,
    channel_id  uuid        not null,

    constraint api_keys_pk          primary key (id),
    constraint api_keys_prefix_ux   unique      (prefix),
    constraint api_keys_channel_fk  foreign key (channel_id) references channels (id)
);