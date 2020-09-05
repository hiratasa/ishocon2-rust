# ISHOCON2-rust

[ISHOCON2](https://github.com/showwin/ISHOCON2)の参照実装のRustへの移植（非公式）。

goの参照実装をベースに、できるだけ処理フローを変えないように実装（ただしDB操作は非同期に変更）。

## ビルド時の注意点

[sqlx](https://github.com/launchbadge/sqlx)のマクロを使用しているため、ビルド時にDBへ接続してスキーマのチェックが走る。

そのため、ビルドするには以下二点が必要:

* DBに実環境同等のテーブルがあること
   * ローカルでの開発時は、ISHOCON2のリポジトリの[init.sql](https://github.com/showwin/ISHOCON2/blob/master/admin/init.sql)もしくは[DBのdump](https://github.com/showwin/ISHOCON2/blob/master/admin/ishocon2.dump.tar.bz2)を使つ
* DBのURLが`.env`ファイルに以下のような形で記入されていること:
    ```
    DATABASE_URL=mysql://ishocon:ishocon@localhost/ishocon2
    ```
    **こちらは実環境でのビルド時にも必要なので注意**