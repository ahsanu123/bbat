CREATE OR ALTER PROCEDURE GetAppliedMigrations
RETURNS (
    migrationId TYPE OF COLUMN SimpleMigrator.migrationId,
    name        TYPE OF COLUMN SimpleMigrator.name,
    description TYPE OF COLUMN SimpleMigrator.description,
    appliedDate TYPE OF COLUMN SimpleMigrator.appliedDate,
    isApplied   TYPE OF COLUMN SimpleMigrator.isApplied
)
AS
BEGIN

    FOR SELECT
        migrationId,
        name,
        description,
        appliedDate,
        isApplied
    FROM SimpleMigrator
    WHERE
        isApplied = true
    ORDER BY
        appliedDate DESC
    INTO
        migrationId,
        name,
        description,
        appliedDate,
        isApplied
    DO
    BEGIN
        SUSPEND;
    END
END
