-- Add migration script here
alter table posts rename column post_description to post_body;
