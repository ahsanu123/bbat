```shell
ah@ah-legion ~/p/b/simple_migrator (main)> ls
Cargo.lock*  Cargo.toml*  src/  target/
ah@ah-legion ~/p/b/simple_migrator (main)> isql-fb
Use CONNECT or CREATE DATABASE to specify a database
SQL> create database "./firebird-tets.fdb" user 'SYSDBA' password 'masterkey';
SQL> SELECT SEC$USER_NAME FROM SEC$USERS;

SEC$USER_NAME
===============================================================
SYSDBA
AH

SQL> ^D⏎

ah@ah-legion ~/p/b/simple_migrator (main)> ls
Cargo.lock*  Cargo.toml*  firebird-tets.fdb  firebird_note.md  src/  target/
ah@ah-legion ~/p/b/simple_migrator (main)> isql-fb -u SYSDBA -p masterkey  localhost://home/ah/projects/bbat/simple_migrator/firebird-tets.fdb
Database: localhost://home/ah/projects/bbat/simple_migrator/firebird-tets.fdb, User: SYSDBA
SQL> grant RDB$ADMIN to ah;
SQL> ^D⏎                                                                                                                                                                          ah@ah-legion ~/p/b/simple_migrator (main)> isql-fb -u AH -p 123  localhost://home/ah/projects/bbat/simple_migrator/firebird-tets.fdb
Database: localhost://home/ah/projects/bbat/simple_migrator/firebird-tets.fdb, User: AH
SQL> ^D⏎

SQL> SELECT *
CON> FROM RDB$USER_PRIVILEGES
CON> WHERE RDB$USER = 'AH';

SQL> SELECT CURRENT_ROLE FROM RDB$DATABASE;

ROLE
===============================================================
NONE
```
