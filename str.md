&strは不変で文字列リテラルともいう　既知のサイズ スタックに積まれる
Stringはヒープにメモリを確保しコンパイル時にサイズが不明なテキストを保持できる型

 - スタックはLIFOのデータ構造をした実行時に利用できるメモリ領域 スタックはLIFOでデータを取り出しやすいため高速
 またスタックに積まれるデータは全て既知の固定サイズのデータ
 - ヒープはサイズが可変なデータを可変するためのメモリ領域 ヒープにデータを置く時は OSに訪ねてOSは十分なサイズの空のメモリ領域を見つけ
 使用中としてその領域のポインタ(メモリ領域内の番地または住所のようなもの)をOSを返す ただしポインタ自体は既知の固定サイズなのでスタックに積まれる ポインタ上の
 データが必要な時はスタックのポインタを追いかける必要がある ヒープは低速

&str -> スライスの一種。　スライスはコレクション内の一連の要素を参照したもの。(https://doc.rust-jp.rs/book-ja/ch04-03-slices.html)
スライスはコンパイル時にサイズが決まらない。スライスは２つのデータでできており　一つは先頭のポインタ もう一つはスライスの長さ
(https://doc.rust-lang.org/std/primitive.slice.html)
readonlyでリサイズ不可

メモリモデル

stackにprealocated read-only memoryにあるbufferのポインタとその長さが格納

![str_mem](/home/suhrkawase/Pictures/str_mem.png)

preallocated read only memory -> 機械語生成時にすでに確保したメモリ領域



String -> ヒープアロケートされる。リサイズ可能

メモリモデル

stackに文字列に関するデータ(ヒープにあるbufferへのポインタ(バイト列へのポインタ)、その容量と長さ)が格納

heapにbufferデータ

![string_memory](/home/suhrkawase/Pictures/string_memory.png)

(https://speakerdeck.com/helloyuk13/rusthanzuon-at-eurekashe?slide=35)

Stringは３つのパーツで出来ている　文字列の中身を保持するメモリ領域へのポインタとその長さとその領域の許容量(ヒープの再割り当てなしで格納できるUTF-8バイト長) これらはスタックに保持
*(https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html)
