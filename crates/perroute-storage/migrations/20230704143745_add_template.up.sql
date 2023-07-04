create table templates(
    id              uuid            not null,
    code            varchar(255)    not null,
    description     varchar(100)    null,
    template        text            not null,
    channel_id         uuid    not null,

    constraint templates_pk primary key (id),
    constraint templates_channel_fk foreign key (channel_id) references channels(id),
    constraint templates_code_channel_un unique (code, channel_id)
)
