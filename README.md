## One to One

sample_b は sample_a が存在しないと、生成できない関係

```mermaid
erDiagram
  sample_a ||--o| sample_b : "one to one"
  sample_a {
    i32 Id PK
    String Value
  }
  sample_b {
    i32 Id PK
    String Value
    i32 SampleAId UK
  }
```

## One to Many

sample_d は sample_c が存在しないと、生成できない関係

```mermaid
erDiagram
  sample_c ||--o{ sample_d : "one to many"
  sample_c {
    i32 Id PK
    String Value
  }
  sample_d {
    i32 Id PK
    String Value
    i32 SampleCId
  }
```

## Many to Many

junction が中間テーブル

```mermaid
erDiagram
  sample_e ||--o{ junction : "one to many"
  sample_f ||--o{ junction : "one to many"
  sample_e {
    i32 Id PK
    String Value
  }
  sample_f {
    i32 Id PK
    String Value
  }
  junction {
    i32 Id PK
    i32 SampleEId
    i32 SampleFId
  }
```

## 参考サイト

https://www.sea-ql.org/SeaORM/docs/relation/one-to-one/

https://www.sea-ql.org/SeaORM/docs/relation/one-to-many/

https://www.sea-ql.org/SeaORM/docs/relation/many-to-many/

https://www.sea-ql.org/SeaORM/docs/basic-crud/select/

https://zenn.dev/icy_mountain/scraps/dd0ff88411efdb
