-- Add migration script here
ALTER TABLE register_links
ADD used_by CHAR(36);