# Migration:
## Postgresql:
```
TODO УБРАТЬ ДИзель до деплоя на продакшн! Написать свое, или найти готовый.

diesel database reset --database-url=postgres://root:password@core_postgresql/tento --migration-dir=/_tento/project/migration/postgresql/core/
diesel database reset --database-url=postgres://root:password@authorization_postgresql/tento --migration-dir=/_tento/project/migration/postgresql/authorization/
```