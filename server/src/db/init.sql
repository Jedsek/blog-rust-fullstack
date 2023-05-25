create table if not exists article (
    id integer primary key autoincrement,
    title varchar(255) not null,
    content text not null,
    date date default (date('now')) not null
);

insert or ignore into article values (1, "Article-1", "This is article-1.", "2023-05-25");
insert or ignore into article values (2, "Article-2", "This is article-2.", "2023-05-26");
insert or ignore into article values (3, "Article-3", "This is article-3.", "2023-05-27");