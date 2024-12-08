## 使用するコマンド
```bash
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

```bash
$ sqlx database create
```

```bash
$ sqlx migrate add -r users
```

```bash
$ sqlx migrate run
```