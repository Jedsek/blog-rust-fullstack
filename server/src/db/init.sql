create table if not exists articles (
    id integer primary key autoincrement,
    title varchar(255) not null,
    content text not null,
    date date default (date('now')) not null
);