# rust_dxlib
DXライブラリをRustで使えるようにする。

使い方

extern crate dxlib_rust;

fn main(){
  //DxライブラリのDLLを読み込んでオブジェクト化
  let lib = dxlib::DxlibDLL::new();
  //あとはこんな感じでdxライブラリの関数呼び出せるよ～
  lib.dx_DxLib_Init();
}
