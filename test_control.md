### Изменение количества тредов

```bash
cargo test -- --test-threads=1
```

### Вывод на консоль корректных тестов

```bash
cargo test -- --nocapture
```

### Запуск одного теста

```bash
cargo test test_name
```

### Запуск нескольких тестов (не всех)

```bash
cargo test part_name
```

### Выполнить только проигнорированные тесты

```bash
cargo test -- --ignored
```