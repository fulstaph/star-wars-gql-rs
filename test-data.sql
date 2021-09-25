insert into starships (name, class) VALUES ('test', 'test2');

insert into starships (name, class)
VALUES ('Millennium Falcon', 'Carellian Freighter');

-- planets
insert into planets (name) VALUES ('Hoth');
insert into planets (name) VALUES ('Kamino');
insert into planets (name) VALUES ('Bespin');
insert into planets (name) VALUES ('Coruscant');
insert into planets (name) VALUES ('Alderaan');


-- characters & friends
insert into characters (name, race, starship_id)
values ('Han Solo', 'human', 2);
insert into characters (name, race, starship_id)
values ('Luke Skywalker', 'human', 2);
insert into characters (name, race, starship_id)
values ('Chewbacca', 'wookie', 2);

insert into friends(character_id, friend_character_id)
 values (1, 2);
insert into friends(character_id, friend_character_id)
 values (1, 3);

-- filmmakers
insert into filmmakers(first_name, last_name, profession)
values ('George', 'Lucas', 'producer');
insert into filmmakers(first_name, last_name, profession)
values ('George', 'Lucas', 'director');
insert into filmmakers(first_name, last_name, profession)
values ('George', 'Lucas', 'scriptwriter');
insert into filmmakers(first_name, last_name, profession)
values ('J. J.', 'Abrams', 'director');
insert into filmmakers(first_name, last_name, profession)
values ('', '', 'producer');

-- movies
insert into movies(title, director_id, scriptwriter_id, producer_id, release_date)
values ('Star Wars: A New Hope', 2, 3, 1, '1977-05-25');
