create table if not exists artical (
    id int primary key,
    title varchar(255),
    content text,
    date date default (date('now'))
);

insert into artical (title, content, date) values ("Artical-1", "This is artical-1.", "2023-05-25");
insert into artical (title, content, date) values ("Artical-2", "This is artical-2.", "2023-05-26");
insert into artical (title, content, date) values ("Artical-3", "This is artical-3.", "2023-05-27");