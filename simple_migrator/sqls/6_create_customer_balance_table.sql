CREATE TABLE CustomerBalance (
    customerId  INT NOT NULL,
    balanceId   INT NOT NULL,
    datetime    TIMESTAMP NOT NULL,
    description VARCHAR(500),

    CONSTRAINT FK_CustomerBalance_Customer_CustomerId
        FOREIGN KEY (customerId)
            REFERENCES Customer(customerId)
            ON DELETE CASCADE
            ON UPDATE CASCADE,

    CONSTRAINT FK_CustomerBalance_Balance_BalanceId
        FOREIGN KEY (balanceId)
            REFERENCES Balance(balanceId)
            ON DELETE CASCADE
            ON UPDATE CASCADE

)