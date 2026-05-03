CREATE OR ALTER PROCEDURE UpsertMigrationStatus (
    migrationId TYPE OF COLUMN SimpleMigrator.migrationId,
    name        TYPE OF COLUMN SimpleMigrator.name,
    description TYPE OF COLUMN SimpleMigrator.description,
    appliedDate TYPE OF COLUMN SimpleMigrator.appliedDate,
    isApplied   TYPE OF COLUMN SimpleMigrator.isApplied
)
RETURNS (affected INT)
AS
    DECLARE dataCount INT;
BEGIN
    SELECT COUNT(*)
    FROM SimpleMigrator
    WHERE
        migrationId = :migrationId
    INTO :dataCount;

    IF (dataCount > 0) THEN
    BEGIN
        UPDATE SimpleMigrator
        SET
            name            = :name,
            description     = :description,
            appliedDate     = :appliedDate,
            isApplied       = :isApplied
        WHERE
            migrationId = :migrationId;
    END

    ELSE
    BEGIN
        INSERT INTO SimpleMigrator (MIGRATIONID, NAME, DESCRIPTION, APPLIEDDATE, ISAPPLIED)
        VALUES (
                :migrationId,
                :name,
                :description,
                :appliedDate,
                :isApplied
        );
    END

    affected = ROW_COUNT;

    SUSPEND;
END