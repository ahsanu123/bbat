CREATE TABLE Customer (
    customerId  INT GENERATED ALWAYS AS IDENTITY NOT NULL PRIMARY KEY,
    userId      INT NOT NULL,

    CONSTRAINT FK_User_UserId
        FOREIGN KEY (userId)
            REFERENCES USERS(userId)
            ON DELETE CASCADE
            ON UPDATE CASCADE
)