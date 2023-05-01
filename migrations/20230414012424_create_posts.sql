-- Add migration script here
-- Add migration script here
create table posts(post_id serial primary key, post_title varchar(100), post_description varchar(300), category_id serial, foreign key(category_id) references category_post)

