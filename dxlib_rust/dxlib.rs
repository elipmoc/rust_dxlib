/*dxlib const variables*/

pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

/*dxlib struct types*/

#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

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
}

/*wrapped function*/
mod hidden {
    use dxlib;
    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "stdcall" {
        pub fn dx_ClearDrawScreen(ClearRect: *mut dxlib::RECT) -> i32;
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
