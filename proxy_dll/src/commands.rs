use muldiv::MulDiv;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, WINDOWINFO};
use windows::Win32::{
    Foundation::HINSTANCE,
    UI::Input::KeyboardAndMouse::{
        mouse_event, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
        MOUSEEVENTF_MOVE,
    },
    UI::WindowsAndMessaging::{FindWindowW, GetWindowInfo, SM_CXSCREEN, SM_CYSCREEN},
};

pub unsafe fn mouse_left_click(_: HINSTANCE, x: i32, y: i32) {
    let hndl = FindWindowW(None, w!("Rangers"));
    let mut win_info = WINDOWINFO::default();
    if GetWindowInfo(hndl, &mut win_info) == false {
        let e = GetLastError();
        eprintln!("error in mouse_left_click: {:?}", e);
        return;
    }

    let x = win_info.rcClient.left + x;
    let y = win_info.rcClient.top + y;

    let sx = GetSystemMetrics(SM_CXSCREEN);
    let sy = GetSystemMetrics(SM_CYSCREEN);
    let x = x.mul_div_round(65535, sx).expect("muldiv x");
    let y = y.mul_div_round(65535, sy).expect("muldiv y");

    mouse_event(
        MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE,
        x,
        y,
        0,
        0,
    );
    std::thread::sleep(std::time::Duration::from_millis(100));
    mouse_event(
        MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE,
        x,
        y,
        0,
        0,
    );
}
