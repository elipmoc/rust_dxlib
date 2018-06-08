extern crate dxlib_rust;
use dxlib_rust::dxlib;

fn main() {
    match game() {
        Ok(_) => (),
        Err(_) => println!("err"),
    };
}

fn game() -> dxlib::DxResult<i32> {
    let lib = dxlib::DxlibDLL::new();
    // DxLibはデフォルトでフルスクリーンなのでウィンドウモードに変更
    lib.dx_ChangeWindowMode(dxlib::TRUE)?;

    // ウィンドウサイズを320x240に変更
    // （32bit, 60fpsを指定していますがウィンドウモードでは関係ないはず）
    lib.dx_SetGraphMode(320, 240, 32, 60)?;

    // DxLibの初期化
    lib.dx_DxLib_Init()?;

    // キー入力かマウス入力を待つ
    lib.dx_WaitKey()?;

    // キー入力かマウス入力を待つ
    lib.dx_WaitKey()?;

    // DxLibの後処理
    lib.dx_DxLib_End()?;
    Ok(1)
}
