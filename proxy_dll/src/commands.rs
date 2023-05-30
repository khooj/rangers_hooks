use muldiv::MulDiv;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::ScreenToClient;
use windows::Win32::System::SystemServices::MK_LBUTTON;
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SendMessageW, WINDOWINFO, WM_LBUTTONDOWN, WM_LBUTTONUP, GetCursorPos, PostMessageW,
};
use windows::Win32::{
    Foundation::HINSTANCE,
    UI::Input::KeyboardAndMouse::{
        mouse_event, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
        MOUSEEVENTF_MOVE,
    },
    UI::WindowsAndMessaging::{FindWindowW, GetWindowInfo, SM_CXSCREEN, SM_CYSCREEN},
};

pub struct AbsolutePoint {
    pub x: i32,
    pub y: i32,
}

impl AbsolutePoint {
    fn normalize(
        &self,
        (game_width, game_height): (i32, i32),
        (game_client_x, game_client_y): (i32, i32),
        (sx, sy): (i32, i32),
    ) -> AbsolutePoint {
        println!(
            "{} {} {} {} {} {}",
            game_width, game_height, game_client_x, game_client_y, sx, sy
        );
        let x = self.x.mul_div_round(game_width, 1280).expect("muldiv x");
        let y = self.y.mul_div_round(game_height, 1024).expect("muldiv y");

        let x = x + game_client_x;
        let y = y + game_client_y;

        let x = x.mul_div_round(65535, sx).expect("muldiv x");
        let y = y.mul_div_round(65535, sy).expect("muldiv y");
        AbsolutePoint { x, y }
    }
}

pub unsafe fn mouse_left_click(point: AbsolutePoint) {
    let hndl = FindWindowW(None, w!("Rangers"));
    let mut win_info = WINDOWINFO::default();
    if GetWindowInfo(hndl, &mut win_info) == false {
        let e = GetLastError();
        eprintln!("error in mouse_left_click: {:?}", e);
        return;
    }

    let sx = GetSystemMetrics(SM_CXSCREEN);
    let sy = GetSystemMetrics(SM_CYSCREEN);

    let point = point.normalize(
        (
            win_info.rcClient.right - win_info.rcClient.left,
            win_info.rcClient.bottom - win_info.rcClient.top,
        ),
        (win_info.rcClient.left, win_info.rcClient.top),
        (sx, sy),
    );

    mouse_event(
        MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE,
        point.x,
        point.y,
        0,
        0,
    );
    std::thread::sleep(std::time::Duration::from_millis(50));
    mouse_event(
        MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE,
        point.x,
        point.y,
        0,
        0,
    );
}

pub fn next_day() {
    unsafe {
        mouse_left_click(AbsolutePoint { x: 1200, y: 1000 });
    }
}

pub unsafe fn mouse_left_click_send(point: AbsolutePoint) {
    let hwnd = FindWindowW(None, w!("Rangers"));
    let mut win_info = WINDOWINFO::default();
    if GetWindowInfo(hwnd, &mut win_info) == false {
        let e = GetLastError();
        eprintln!("error in mouse_left_click_send find window: {:?}", e);
        return;
    }

    let sx = GetSystemMetrics(SM_CXSCREEN);
    let sy = GetSystemMetrics(SM_CYSCREEN);

    let point = point.normalize(
        (
            win_info.rcClient.right - win_info.rcClient.left,
            win_info.rcClient.bottom - win_info.rcClient.top,
        ),
        (win_info.rcClient.left, win_info.rcClient.top),
        (sx, sy),
    );

    let mut p = POINT {
        x: point.x,
        y: point.y,
    };
    // if GetCursorPos(&mut p) == false {
    //     let e = GetLastError();
    //     eprintln!("error in mouse_left_click_send getcursorpos: {:?}", e);
    //     return;
    // }
    // let mut coord = LPARAM(coord);
    if ScreenToClient(hwnd, &mut p).0 == 0 {
        let e = GetLastError();
        eprintln!("error in mouse_left_click_send screentoclient: {:?}", e);
        return;
    }

    let coord = (p.y as isize) << 16 | p.x as isize;
    println!("x: {:x} y: {:x} coord: {:x}", p.x, p.y, coord);
    let result = PostMessageW(hwnd, WM_LBUTTONDOWN, WPARAM(1), LPARAM(coord));
    if result.0 == 0 {
        let e = GetLastError();
        eprintln!("error in mouse_left_click_send sendmessage 1: {:?}", e);
        return;
    }
    // std::thread::sleep(std::time::Duration::from_millis(50));
    let result = PostMessageW(hwnd, WM_LBUTTONUP, WPARAM(1), LPARAM(coord));
    if result.0 == 0 {
        let e = GetLastError();
        eprintln!("error in mouse_left_click_send sendmessage 2: {:?}", e);
        return;
    }
}

pub fn next_day_new() {
    unsafe {
        mouse_left_click_send(AbsolutePoint { x: 10, y: 10 });
    }
}
