use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::SendInput;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT_MOUSE;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_LEFTDOWN;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_LEFTUP;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEINPUT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

fn main() {
    println!("Waiting for 2 seconds...");
    sleep(Duration::from_secs(2));

    let p1 = get_mouse_position();
    println!("First position captured: {:?}", p1);

    println!("Waiting for another 2 seconds...");
    sleep(Duration::from_secs(2));

    let p2 = get_mouse_position();
    println!("Second position captured: {:?}", p2);

    // Calculate the center and radius of the circle
    let center = ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0);
    let dx = p1.0 - center.0;
    let dy = p1.1 - center.1;
    let radius = (dx * dx + dy * dy).sqrt();

    println!("Circle center: {:?}, Radius: {}", center, radius);

    // Calculate the initial angle theta0 so that the circle starts at p1
    let theta0 = (dy).atan2(dx); // atan2(dy, dx)

    // Move mouse back to p1
    set_mouse_position(p1.0 as i32, p1.1 as i32);

    // Press the left mouse button to start drawing
    mouse_left_press();

    // Simulate drawing the circle starting from theta0
    let steps = 360; // Number of points along the circle
    for i in 0..=steps {
        let theta = theta0 + i as f64 * (2.0 * std::f64::consts::PI) / steps as f64;
        let x = center.0 + radius * theta.cos();
        let y = center.1 + radius * theta.sin();

        set_mouse_position(x as i32, y as i32);

        // Sleep briefly to make the movement smooth
        sleep(Duration::from_millis(5));
    }

    // Release the left mouse button to finish drawing
    mouse_left_release();

    println!("Circle drawing complete!");
}

fn get_mouse_position() -> (f64, f64) {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point);
        (point.x as f64, point.y as f64)
    }
}

fn set_mouse_position(x: i32, y: i32) {
    unsafe {
        SetCursorPos(x, y);
    }
}

fn mouse_left_press() {
    unsafe {
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTDOWN,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

fn mouse_left_release() {
    unsafe {
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}
