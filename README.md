# rust_dxlib
DXライブラリをRustで使えるようにする。

使い方

extern crate dxlib_rust;

```Rust
fn main(){
  //DxライブラリのDLLを読み込んでオブジェクト化
  let lib = dxlib::DxlibDLL::new();
  //あとはこんな感じでdxライブラリの関数呼び出せるよ～
  lib.dx_DxLib_Init();
}
```

あとはコンパイルしてできた生成物と同じ階層にDxLib_x64.dllを配置すると動く。

DxLib_x64.dllはDxライブラリ公式サイトのダウンロードページからC#版のDXライブラリをダウンロードすると手に入る
