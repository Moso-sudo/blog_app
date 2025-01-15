-- Your SQL goes here
create table users(
 id serial primary key,
 name character varying (150) not null,
 email text not null,
 password text not null,
 updated_at timstamp  without_time zone default CURRENT_TIMESTAMP not null,
 created_at timstamp  without_time zone default CURRENT_TIMESTAMP not null,

)
   

