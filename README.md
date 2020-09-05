# ISHOCON2-rust

[ISHOCON2](https://github.com/showwin/ISHOCON2)の参照実装のRustへの移植（非公式）。

goの参照実装をベースに、できるだけ処理フローを変えないように実装。

## ローカルでの開発時の注意点

[sqlx](https://github.com/launchbadge/sqlx)のマクロを使用しているため、ビルド時にDBへ接続してスキーマのチェックが走る。

この際、ローカルのDBに実環境同等のテーブルがなければコンパイルに失敗する（スキーマだけ一致していればよく、中身のデータは不要）。

1. DBにテーブルを用意
   * ISHOCON2のリポジトリの[init.sql](https://github.com/showwin/ISHOCON2/blob/master/admin/init.sql)もしくは[DBのdump](https://github.com/showwin/ISHOCON2/blob/master/admin/ishocon2.dump.tar.bz2)を使つ
1. DBのURLを`.env`ファイルに以下のような形で記入する:
    ```
    DATABASE_URL=mysql://ishocon:ishocon@localhost/ishocon2
    ```