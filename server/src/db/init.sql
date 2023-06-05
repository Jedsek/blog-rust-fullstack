create table if not exists articles (
    id integer primary key autoincrement,
    title varchar(255) not null,
    content text not null,
    date date default (date('now')) not null
);

create table if not exists users (
    id integer primary key,
    name varchar(255) not null,
    avatar_url varchar(255) not null
);

create table if not exists comments (
    id integer primary key autoincrement,
    user_id integer,
    article_id integer,
    content varchar(255),
    date date default (date('now')) not null,
    foreign key(user_id) references users(id)
    foreign key(article_id) references articles(id)
);