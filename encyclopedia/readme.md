# Migration
## Postgresql
### Production environment
```
TODO Написать, как работать с миграциями
```
### Development local environment
```
diesel database reset --database-url=postgres://root:password@core_postgresql/tento --migration-dir=/_tento/project/migration/postgresql/core/
diesel database reset --database-url=postgres://root:password@database_2_postgresql/tento --migration-dir=/_tento/project/migration/postgresql/database_2/
```

TODO УБРАТЬ ДИзель до деплоя на продакшн! Написать свое, или найти готовый.