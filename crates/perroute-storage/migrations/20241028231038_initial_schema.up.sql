create table business_units (
    id              varchar(21) primary key,
    code            varchar(50) not null,
    name            text not null,
    vars            jsonb not null,
    created_at      timestamp not null,
    updated_at      timestamp not null,
    constraint business_unit_code_idx unique (code)
);
