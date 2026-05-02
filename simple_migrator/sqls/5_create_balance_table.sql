CREATE TABLE Balance (
    balanceId       INT GENERATED ALWAYS AS IDENTITY NOT NULL PRIMARY KEY,
    balanceValue    INT NOT NULL
)

CREATE TABLE PaymentBalance (
    balanceId   INT NOT NULL,
    paymentId   INT NOT NULL,

    CONSTRAINT FK_Balance_BalanceId
        FOREIGN KEY (balanceId)
            REFERENCES Balance(balanceId)
            ON UPDATE CASCADE
            ON DELETE CASCADE,

    CONSTRAINT FK_Payment_PaymentId
        FOREIGN KEY (paymentId)
            REFERENCES Payment(paymentId)
            ON UPDATE CASCADE
            ON DELETE CASCADE
)