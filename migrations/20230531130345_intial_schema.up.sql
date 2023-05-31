CREATE TABLE public.channels (
	id uuid NOT NULL,
	code varchar(30) NOT NULL,
	escription varchar NULL,
	CONSTRAINT channels_pk PRIMARY KEY (id),
	CONSTRAINT channels_code UNIQUE (code)
);