-- Your SQL goes here
CREATE TABLE `dregs`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`username` TEXT NOT NULL,
	`user_id` INTEGER NOT NULL,
	`count` INTEGER NOT NULL,
	`paid` BOOL NOT NULL,
	`taken_time` DATE NOT NULL,
	`production_time` DATE NOT NULL,
	`amount` INTEGER NOT NULL,
	`price` INTEGER NOT NULL,
	`description` TEXT NOT NULL
);

CREATE TABLE `posts`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`title` TEXT NOT NULL,
	`body` TEXT NOT NULL,
	`published` BOOL NOT NULL,
	`description` TEXT NOT NULL
);

