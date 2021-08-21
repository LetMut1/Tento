Initialize Postgresql structure from migration. (Need to execute this command in directory containing diesel.toml file):
    diesel database reset --database-url=postgres://root:password@postgresql/mem_is --migration-dir=/usr/local/MemIs/application/source/migration/postgresql/
    