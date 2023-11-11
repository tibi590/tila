CREATE TABLE users (
    id int NOT null PRIMARY KEY AUTO_INCREMENT,
    username varchar(50) NOT null UNIQUE,
    password varchar(50) NOT null
)
