# Migration
## Postgresql
### Production environment
```
<!-- TODO Написать, как работать с миграциями -->
```
### Development local environment
```
diesel database reset --database-url=postgres://root:password@database_1_postgresql/tento --migration-dir=/_tento/application/migration/postgresql/database_1/
diesel database reset --database-url=postgres://root:password@database_2_postgresql/tento --migration-dir=/_tento/application/migration/postgresql/database_2/
```

<!-- TODO УБРАТЬ ДИзель до деплоя на продакшн! Написать свое, или найти готовый. -->


TODO:
ФРОНТЕНД ОПТИмИзация:

1) Переиспользованеи коннекшна для http:
То есть, установили несколько коннекшнов, для запросов асинхронно. Затем держим некоторые коннекшны открытыми для последующих запросов, но не слишком долго. То есть, если ситуация уже не предполагает постоянного взаимодействия по коннекшну, его нужно закрыть.
https://stackoverflow.com/questions/58819199/how-to-keep-long-connection-in-http2
