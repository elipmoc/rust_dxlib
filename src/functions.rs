use std::ffi::CString;
use std::os::raw::c_char;

use const_variables::*;
use types::*;

/*dxlib function extern declaration*/

#[link(name = "DxLib_x64")]
#[no_mangle]
extern "stdcall" {
    pub fn dx_DxLib_Init() -> i32;
    pub fn dx_ChangeWindowMode(flag: i32) -> i32;
    pub fn dx_DxLib_End() -> i32;
    pub fn dx_ProcessMessage() -> i32;
    pub fn dx_SetMainWindowText(WindowText: *const u8) -> i32;
    pub fn dx_SetGraphMode(SizeX: i32, SizeY: i32, ColorBitNum: i32, RefreshRate: i32) -> i32;
    pub fn dx_ScreenFlip() -> i32;
    pub fn dx_WaitKey() -> i32;
    pub fn dx_DrawGraph(x: i32, y: i32, GrHandle: i32, TransFlag: i32) -> i32;
    pub fn dx_SetDrawScreen(DrawScreen: i32) -> i32;
    pub fn dx_GetColor(Red: i32, Green: i32, Blue: i32) -> Color;
}

/*wrapped function*/
mod hidden {
    use std::os::raw::c_char;
    use types::*;
    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "stdcall" {
        pub fn dx_ClearDrawScreen(ClearRect: *mut RECT) -> i32;
        pub fn dx_LoadGraph(FileName: *const c_char, NotUse3DFlag: i32) -> i32;

    }
}

/*wrap function*/
pub fn dx_ClearDrawScreen() -> i32 {
    let mut tmp = RECT {
        left: -1,
        right: -1,
        top: -1,
        bottom: -1,
    };
    unsafe { hidden::dx_ClearDrawScreen(&mut tmp) }
}

pub fn dx_LoadGraph(FileName: &str) -> i32 {
    unsafe { hidden::dx_LoadGraph(CString::new(FileName).unwrap().as_ptr(), FALSE) }
}
