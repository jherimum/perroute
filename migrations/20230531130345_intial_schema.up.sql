CREATE TABLE public.channels (
	id uuid NOT NULL,
	code varchar(30) NOT NULL,
	name varchar(100) NULL,
	CONSTRAINT channels_pk PRIMARY KEY (id),
	CONSTRAINT channels_code UNIQUE (code)
);