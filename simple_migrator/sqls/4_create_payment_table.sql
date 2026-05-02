CREATE TABLE Payment (
    paymentId   INT GENERATED ALWAYS AS IDENTITY NOT NULL PRIMARY KEY,
    fromDate    TIMESTAMP NOT NULL,
    toDate      TIMESTAMP NOT NULL,
    total       INT NOT NULL,
    bill        INT NOT NULL
)

CREATE TABLE PaymentRecord (
    paymentId   INT NOT NULL,
    recordId    INT NOT NULL,

    CONSTRAINT FK_Record_RecordId
        FOREIGN KEY (recordId)
            REFERENCES Records(recordId)
            ON DELETE CASCADE
            ON UPDATE CASCADE
)