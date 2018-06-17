extern crate rust_dxlib;
use rust_dxlib::*;

fn main() {
    game();
}

fn game() {
    unsafe {
        //ウィンドウモード変更と初期化と裏画面設定
        dx_ChangeWindowMode(TRUE);
        dx_DxLib_Init();
        dx_SetDrawScreen(DX_SCREEN_BACK);

        // while( 裏画面を表画面に反映, メッセージ処理, 画面クリア )
        while (dx_ScreenFlip() == 0 && dx_ProcessMessage() == 0 && dx_ClearDrawScreen() == 0) {
            dx_DrawLine(10, 10, 500, 500, dx_GetColor(255, 255, 255), 10);
            dx_DrawLineAA(100.3, 10.4, 500.550, 500.4, dx_GetColor(255, 255, 255), 2.6);
            dx_DrawBox(80, 80, 200, 150, dx_GetColor(255, 200, 255), FALSE);
            dx_DrawBox(200, 200, 250, 300, dx_GetColor(0, 255, 255), TRUE);
            dx_DrawBoxAA(
                100.0,
                30.0,
                200.0,
                30.0,
                dx_GetColor(0, 255, 255),
                TRUE,
                1.0,
            );
            dx_DrawCircle(150, 150, 20, dx_GetColor(255, 255, 0), TRUE, 5);
            dx_DrawCircleAA(
                120.3,
                100.2,
                20.0,
                100,
                dx_GetColor(255, 255, 0),
                FALSE,
                5.0,
            );
            dx_DrawOval(320, 240, 150, 100, dx_GetColor(0, 0, 255), TRUE, 3);
            dx_DrawTriangle(520, 100, 550, 420, 480, 420, dx_GetColor(0, 255, 255), TRUE);
        }

        dx_DxLib_End(); // DXライブラリ終了処理
    }
}
