# Migration:
## Postgresql:
```
diesel database reset --database-url=postgres://root:password@postgresql/memis --migration-dir=/usr/local/memis/application/source/migration/postgresql/
```