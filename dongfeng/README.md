# Introduction 

Learning rust database  with diesel.

## Diesel  Workflow Note

run `diesel migration generate --diff-schema <name>`  to generate migration, based  on schema and current database.
by using generate `--diff-schema` diesel will automatically  create migration  script based on current `schema.rs`.

## Step Define Table (Diesel Method Is A Bit Manual)

diesel cli will check for difference between your defined schema and table inside your database, so if you want to full update 
you need to drop/delete database, then run `diesel setup`, that command will create new database based your setting 
and create initial sql based your schema.

after migration was generated, you can run `diesel migration run`, that command will migrate all un-migrated table. 

you can list all migration with `diesel migration list`.

so basically to manage your database, you can define your table inside your schema, then sync it on your model.
after that you can run `diesel migration generate <name>`, then `diesel migration run` to run all needed migration

## Reference  

- [Diesel Documentation](https://diesel.rs/guides/getting-started)
- [Configuring Diesel Cli](http://diesel.rs/guides/configuring-diesel-cli)
