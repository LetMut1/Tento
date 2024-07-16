# System design
## Way
В условиях неизвестной ожидаемой нагрзуки, когда необходимо проверить бизнес-модель, "преждевременная оптимизация принесет лишь вред", поэтому изначальный дизайн использует в себе прямые явные пути хранения и обработки данных, и лишь затем все начинает оптимизироваться.
## Now
Используется одна БД (Постгрескл) для всех нужд. Инстансы Бд распределены по ожидаемой при хайлоаде нагрузке и бизнес-ответстсвенности и так, чтобы можно было без потерять заменить одну Бд на другую - например, хранение и обработка Лайков/Реакций отделено от хранения и обрабокти Комментариев.

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