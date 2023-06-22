create table messsage_types(
    id          uuid            not null,
    code        varchar(50)     not null,    
    description varchar(500)    not null,
    enabled     boolean         not null,
    channel_id  uuid            not null,    
    CONSTRAINT message_types_pk PRIMARY KEY (id),
    CONSTRAINT message_types_code UNIQUE (code),
    CONSTRAINT message_types_channel_fk FOREIGN KEY (channel_id) REFERENCES channels(id)
)
