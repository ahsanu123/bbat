CREATE TABLE Prices (
    priceId     INT GENERATED ALWAYS AS IDENTITY NOT NULL PRIMARY KEY,
    priceValue  INT NOT NULL,

    CONSTRAINT Chk_Minimum_Price CHECK (
        priceValue > 5000
    )
)