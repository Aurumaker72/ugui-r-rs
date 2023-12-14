use crate::core::geo::{Point, Rect};
use crate::core::styles::Styles;
use crate::window::Window;
use crate::HWND;

/// Tries to get the window at a specified point
/// The window with the highest Z-order is preferred
///
/// # Arguments
///
/// * `windows`: A slice containing the windows
/// * `point`: The point to look for windows at
///
/// returns: Option<&Window> The window at the specified point, or None if no window is at the specified point
pub fn window_at_point(windows: &[Window], point: Point) -> Option<&Window> {
    if let Some(control) = windows
        .iter()
        .rev()
        .find(|x| point.inside(x.rect) && x.styles.contains(Styles::Visible))
    {
        return Some(control);
    }
    return None;
}

/// Tries to get the window with the specified handle
///
/// # Arguments
///
/// * `windows`: A slice containing the windows
/// * `hwnd`: The window handle to look for
///
/// returns: Option<&Window> The window with the specified handle, or None if no window is at the specified point
pub fn window_from_hwnd_safe(windows: &[Window], hwnd: HWND) -> Option<&Window> {
    windows.iter().find(|x| x.hwnd == hwnd)
}

/// Gets the window with the specified handle, panicking if none has the specified handle
///
/// # Arguments
///
/// * `windows`: A slice containing the windows
/// * `hwnd`: The window handle to look for
///
/// returns: &Window The window with the specified handle
pub fn window_from_hwnd(windows: &[Window], hwnd: HWND) -> &Window {
    if let Some(window) = windows.iter().find(|x| x.hwnd == hwnd) {
        return window;
    }
    panic!("No window with specified HWND found");
}

/// Gets a mutable reference to the window with the specified handle, panicking if none has the specified handle
///
/// # Arguments
///
/// * `windows`: A slice containing the windows
/// * `hwnd`: The window handle to look for
///
/// returns: &mut Window The window with the specified handle
pub fn window_from_hwnd_mut(windows: &mut [Window], hwnd: HWND) -> &mut Window {
    for i in 0..windows.len() {
        if windows[i].hwnd == hwnd {
            return &mut windows[i];
        }
    }
    panic!("No window with specified HWND found");
}

/// Clears a dependent optional handle if it points to a non-candidate control
/// A non-candidate control is characterized by it being inappropriate to point a focus handle to the control
///
/// # Arguments
///
/// * `windows`: A slice containing the windows
/// * `hwnd`: The window handle to fix
///
/// returns: Option<usize> The fixed handle
pub fn fix_dependent_visual_handle(windows: &[Window], hwnd: Option<HWND>) -> Option<HWND> {
    if hwnd.is_none() {
        return hwnd;
    }

    let window = window_from_hwnd_safe(windows, hwnd.unwrap());

    if window.is_none()
        || !window.unwrap().styles.contains(Styles::Enabled)
        || !window.unwrap().styles.contains(Styles::Visible)
    {
        println!("Dependent handle points to inappropriate control");
        return None;
    }
    return hwnd;
}

pub fn get_windows_inside_rect(windows: &[Window], rect: Rect) -> Vec<&Window> {
    let mut rects: Vec<&Window> = Default::default();
    for window in windows {
        if window.rect.top_left().inside(rect)
            || window.rect.top_right().inside(rect)
            || window.rect.bottom_left().inside(rect)
            || window.rect.bottom_right().inside(rect)
        {
            rects.push(window);
        }
    }
    rects
}
