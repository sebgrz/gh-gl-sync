create table if not exists project (
	id serial primary key,
	name varchar(200) not null,
	ssh_url varchar(1000) not null,
	created_on timestamp not null
);

