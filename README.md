# rust-kifuwarabe-wcsc29-lib
コンピューター将棋ソフトきふわらべ 第29回世界コンピューター将棋選手権に向けたバージョン☆（＾～＾）

## 用語集

### ［テープ・ボックス］
    ０～Ｎ個の棋譜を保存した JSON形式テキスト・ファイル。拡張子は .rbox。
    ファイルと テープ・ボックスは　１対１対応　する。
    ファイル名はランダムに生成される。

### ［テープ］
    棋譜。１対局分。
    拡張子 .tapefrag のテキストファイルに保存される。
    JSON形式でない不完全なファイルであることを強調して、「テープ・フラグメント」と呼ぶことがある。
    ファイル名はランダムに生成される。

### ［カセット・デッキ］
    きふわらべ は、カセット・デッキ を１つ持っている。
    カセット・デッキには ２本のテープを持たせることができて、
    それぞれのテープは、トレーニング・テープ、ラーニング・テープ　という役割で呼ばれる。

    トレーニング・テープ　は無しでも構わない。
    ラーニング・テープは　ＰＣでも壊れていない限り　必ず補充される。
     
### ［トレーニング・テープ（役割）］
    略称 t_tape。
    読込専用。読み込んだ棋譜などが　これにあたる。

### ［ラーニング・テープ（役割）］
    略称 l_tape。
    毎回、空っぽのテープから始まり、新しい棋譜を記録するのに使う。

### ［トレーニング・フォルダー］
    設定ファイルで決めておいたフォルダー。
    テープ・ボックスが０～Ｎ個置いてある。
    きふわらべ がこのフォルダーの中から　トレーニング・テープ　を勝手に探す。

### ［ラーニング・フォルダー］
    設定ファイルで決めておいたフォルダー。
    テープ・ボックスが０～Ｎ個置いてある。
    きふわらべは、このフォルダーの中に ラーニング・テープ を勝手に保存する。

### ［テープ・フラグメント・フォルダー］
    「棋譜収集」フェーズで利用する。
    設定ファイルで決めておいたフォルダー。
    テープ・フラグメントが０～Ｎ個置いてある。
    きふわらべは、このフォルダーの中に 指定の名前で テープ・フラグメント を保存する。

## 環境設定。

棋譜を読み込むために、以下の１０個のディレクトリーを作れ。これらのディレクトリーは設定ファイルに記入しろ。
RPM形式棋譜というのは きふわらべ に読める棋譜の形式(Reversible physical move)だぜ。

| 呼び名           | パスの例                             | 働き                                                             |
| ---------------- | ------------------------------------ | ---------------------------------------------------------------- |
| expansion.go     | C:/shogi-record/go/hunting           | ここに置いてあるものはテキストファイルになるまで解凍する。       |
| expansion.went   | C:/shogi-record/went/hunted          | 解凍したらここへ退避。                                           |
| expansion.output | C:/shogi-record/go/cooking           | 解凍した中身はここへ出力。                                       |
| formation.go     | ※expansion.outputと同じ              | ここに置いてあるテキストファイルは UTF8 エンコーディングにする。 |
| formation.went   | C:/shogi-record/went/cooked          | あとはパターンを感じろ。                                         |
| formation.output | C:/shogi-record/go/eating            |                                                                  |
| eating.go        | ※formation.outputと同じ              | ここに置いてあるものは RPM形式棋譜に変換する。                   |
| eating.went      | C:/shogi-record/went/ate             | 感じろ。                                                         |
| eating.output    | C:/muzudho/shogi-record/rpm          | ここに成果物を置く。実践では使わない。                           |
| learning         | C:/muzudho/shogi-record/learning/rpm | 実践で対局した棋譜を置く。実践では使わない。                     |
| training         | C:/muzudho/shogi-record/rpm-json     | 実践で使う。使いたいRPM棋譜をここへ移動しろだぜ。                |

棋譜の変換（翻訳）は Rust言語 の examples に入っている。
これはライブラリなので実行できないので、 kifuwarabe-wcsc29.exe の方を呼び出せだぜ。
解凍や エンコーディング、全自動翻訳は `CsaOpener` といった感じの名前の適当に作った C#言語 のプログラムから Rust言語を叩いてやる。

## How to eat .kif record?

```Shell
### Example.
cd C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29-lib
cls

### Compile.
set RUST_BACKTRACE=1
cargo clippy --example eat_a_kif
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example eat_a_kif -- --path C:/muzudho/kifuwarabe-wcsc29-learn/output-wcsc-record/copied-daiwa/daiwa.kif
```

## How to convert .kif record?

```Shell
### Example.
cd C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29-lib
cls

### Compile.
set RUST_BACKTRACE=1
cargo clippy --example conv_a_kif

### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example conv_a_kif -- --input "C:/shogi-record/go/eating/wcsc/永世名人/01eis-kak.kif" --output "C:/muzudho/shogi-record/rpm/wcsc/永世名人/01eis-kak.rbox"
cargo run --example conv_a_rec -- --input "C:/muzudho/wcsc29-master/shogi-record/eating-go/formation-go$%wcsc28_kifu$%WCSC28_F4_DGK_TNK.csa" --output "C:/muzudho/wcsc29-master/shogi-record/eating-go/formation-go$%wcsc28_kifu$%WCSC28_F4_DGK_TNK.tameshi" --debug
```

## How to eat .csa record?

```Shell
### Example.
cd C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
set RUST_BACKTRACE=1
cargo clippy --example eat_a_csa
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example eat_a_csa -- --path C:/muzudho/kifuwarabe-wcsc29-learn/output-wcsc-record/extracted-wcsc28_kifu/wcsc28_kifu/WCSC_F1_APR_MCB.csa

cargo run --example eat_a_csa -- --path C:/muzudho/kifuwarabe-wcsc29-learn/csa-record/WCSC_F2_QHA_TNK.csa

cargo run --example eat_a_csa -- --path C:/shogi-record/formation-go/wcsc28_kifu/WCSC_F1_APR_MCB.csa

cargo run --example eat_a_csa -- --path C:/shogi-record/eating-went/eating-go/WCSC28_F4_DGK_TNK.csa

cargo run --example eat_a_csa -- --path C:/shogi-record/eating-went/formation-go$%wcsc28_kifu$%WCSC28_F4_HNW_PAL.csa
```

Output.
`C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/libkifuwarabe-wcsc29.rlib`

## How to convert usi-record to rpm-record?

```Shell
### Example.
cd C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
set RUST_BACKTRACE=1
cargo clippy --example usi_to_rpm
 
### Run.
### '--' is separator. You can pass arguments to exe.
### Please change encoding to UTF-8.
cargo run --example usi_to_rpm -- --path C:/muzudho/kifuwarabe-wcsc29-learn/usi-record/test.usi
```

## How to use engine?

[rust-kifuwarabe-wcsc29.](https://github.com/muzudho/kifuwarabe-wcsc29)

ディレクトリ構成を設定すること。トレーニング・ファイルが必要。

## Test

```Shell
### Example.
cd C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
set RUST_BACKTRACE=1
cargo clippy --example main
cargo build --release
 
### Run.
cargo run --example main
```

## Learn

```
### Example.
cd C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29-lib
cls
 
### Compile.
cargo clippy --example learn
 
### Run.
cargo run --example learn
```
