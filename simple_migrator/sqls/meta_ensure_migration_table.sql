CREATE OR ALTER PROCEDURE EnsureMigrationTable
RETURNS (Created BOOLEAN)
AS
    DECLARE TableCount INT;
BEGIN
    SELECT COUNT(*)
    FROM RDB$RELATIONS
    WHERE RDB$RELATION_NAME = 'SIMPLEMIGRATOR'
        INTO :TableCount;

    IF (TableCount = 0) THEN
    BEGIN
        EXECUTE STATEMENT '
        CREATE TABLE SimpleMigrator (
            migrationId     VARCHAR(500) NOT NULL PRIMARY KEY,
            name            VARCHAR(250),
            description     VARCHAR(500),
            appliedDate     TIMESTAMP,
            isApplied       BOOLEAN
        )';

        Created = true;
    END

    ELSE
    BEGIN
       Created = TableCount > 0;
    END

    SUSPEND;
END