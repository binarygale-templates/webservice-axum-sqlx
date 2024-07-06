create table example (
  uuid uuid primary key default uuid_generate_v4(),

  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone not null default now(),

  content text
);
select manage_updated_at('example');

insert into example (content) values ('hello, world');
