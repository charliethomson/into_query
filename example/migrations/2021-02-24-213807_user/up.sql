-- Your SQL goes here

create table if not exists "users" (user_id serial unique not null,
                                                          username text not null,
                                                                        display_name text not null default 'User',
                                                                                                           primary key (user_id));