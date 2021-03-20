use winapi::um::winuser::{WM_RBUTTONDOWN, PostMessageW};
use std::thread::sleep;
use std::time::Duration;
use longpress::window::Window;

fn main() {
    let default_window = Window::get_foreground();
    println!("> Send right click to {{target}}.");
    println!("> The next focused window will be the target.");
    println!();

    let target = target_the_next_focused_window(default_window);
    println!("> target: {}", target.title);
    println!("> Right-click starts when you focus on another window.");
    println!();
    start_when_window_focus_moves_to_another(target);
    println!("> started.")
}

fn target_the_next_focused_window(current: Window) -> Window {
    loop {
        let window = Window::get_foreground();
        if window.hwnd != current.hwnd && window.title != "" {
            return window;
        }
    }
}
fn start_when_window_focus_moves_to_another(target: Window) {
    loop {
        let next = Window::get_foreground();
        if target.hwnd != next.hwnd{
            sleep(Duration::from_millis(100));
            unsafe {
                PostMessageW(target.hwnd, WM_RBUTTONDOWN, 0, 0);
            }
            break;
        }
    }
}
