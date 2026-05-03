CREATE OR ALTER PROCEDURE DeleteMigrationStatus (
    migrationId INT NOT NULL
)
RETURNS (deletedCount INT)
AS
BEGIN
    DELETE FROM SimpleMigrator
    WHERE MigrationId = :migrationId;

    deletedCount = ROW_COUNT;

    SUSPEND;
END