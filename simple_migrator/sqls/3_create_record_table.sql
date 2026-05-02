CREATE TABLE Records (
    recordId    INT GENERATED ALWAYS AS IDENTITY NOT NULL PRIMARY KEY,
    amount      INT NOT NULL,
    priceId     INT NOT NULL,

    CONSTRAINT FK_Price_PriceId
        FOREIGN KEY (priceId)
            REFERENCES Prices(priceId)
            ON DELETE CASCADE
            ON UPDATE CASCADE
)