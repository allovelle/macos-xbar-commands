use accessibility_sys::{
    AXUIElementCopyAttributeValue, AXUIElementCreateApplication,
    AXUIElementPerformAction, AXUIElementRef, kAXChildrenAttribute,
    kAXCloseButtonAttribute, kAXDescriptionAttribute, kAXIdentifierAttribute,
    kAXMainWindowAttribute, kAXPressAction,
};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use core_foundation::array::CFArray;
use core_foundation::base::{CFType, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use objc::{class, msg_send, sel, sel_impl};
use std::{ptr, thread, time::Duration};

fn main()
{
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // Open System Settings
        let ws: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let url_str = NSString::alloc(nil).init_str(
            "x-apple.systempreferences:com.apple.Trackpad-Settings.extension",
        );
        let url: id = msg_send![class!(NSURL), URLWithString: url_str];
        let _: bool = msg_send![ws, openURL: url];

        thread::sleep(Duration::from_secs(1));

        // Get System Settings PID
        let bundle_id =
            NSString::alloc(nil).init_str("com.apple.systempreferences");
        let apps: id = msg_send![class!(NSRunningApplication), runningApplicationsWithBundleIdentifier: bundle_id];
        let count: usize = msg_send![apps, count];
        if count == 0
        {
            panic!("System Settings not running");
        }

        let app: id = msg_send![apps, firstObject];
        let pid: i32 = msg_send![app, processIdentifier];

        // Get main window
        let ax_app = AXUIElementCreateApplication(pid);
        let action = CFString::new(kAXMainWindowAttribute);
        let window =
            retry(3.0, || get_attr(ax_app, action.as_concrete_TypeRef()))
                .expect("No window");

        // Find and click "Scroll & Zoom"
        let scroll_zoom = retry(3.0, || find_by_desc(window, "Scroll & Zoom"))
            .expect("No tab");
        click(scroll_zoom);
        thread::sleep(Duration::from_millis(200));

        // Find and click toggle
        let toggle =
            retry(3.0, || find_by_id(window, "NaturalScrollingToggle"))
                .expect("No toggle");
        click(toggle);
        thread::sleep(Duration::from_millis(100));

        // Close window
        let action = CFString::new(kAXCloseButtonAttribute);
        if let Some(btn) = get_attr(window, action.as_concrete_TypeRef())
        {
            click(btn);
        }
    }
}

unsafe fn get_attr(
    elem: AXUIElementRef,
    attr: CFStringRef,
) -> Option<AXUIElementRef>
{
    unsafe {
        let mut val = ptr::null();
        if AXUIElementCopyAttributeValue(elem, attr, &mut val) == 0
            && !val.is_null()
        {
            Some(val as AXUIElementRef)
        }
        else
        {
            None
        }
    }
}

unsafe fn get_str_attr(
    elem: AXUIElementRef,
    attr: CFStringRef,
) -> Option<String>
{
    unsafe {
        let mut val = ptr::null();
        if AXUIElementCopyAttributeValue(elem, attr, &mut val) == 0
            && !val.is_null()
        {
            let cf_str = CFString::wrap_under_create_rule(val as _);
            Some(cf_str.to_string())
        }
        else
        {
            None
        }
    }
}

unsafe fn get_children(elem: AXUIElementRef) -> Vec<AXUIElementRef>
{
    unsafe {
        let mut val = ptr::null();
        let action = CFString::new(kAXChildrenAttribute);
        if AXUIElementCopyAttributeValue(
            elem,
            action.as_concrete_TypeRef(),
            &mut val,
        ) == 0
            && !val.is_null()
        {
            let arr = CFArray::<CFType>::wrap_under_create_rule(val as _);
            (0 .. arr.len())
                .filter_map(|i| {
                    arr.get(i).map(|t| t.as_CFTypeRef() as AXUIElementRef)
                })
                .collect()
        }
        else
        {
            vec![]
        }
    }
}

unsafe fn find_by_desc(
    elem: AXUIElementRef,
    desc: &str,
) -> Option<AXUIElementRef>
{
    unsafe {
        let action = CFString::new(kAXDescriptionAttribute);
        if let Some(d) = get_str_attr(elem, action.as_concrete_TypeRef())
            && d == desc
        {
            return Some(elem);
        }
        for child in get_children(elem)
        {
            if let Some(found) = find_by_desc(child, desc)
            {
                return Some(found);
            }
        }
        None
    }
}

unsafe fn find_by_id(elem: AXUIElementRef, id: &str) -> Option<AXUIElementRef>
{
    unsafe {
        let action = CFString::new(kAXIdentifierAttribute);
        if let Some(i) = get_str_attr(elem, action.as_concrete_TypeRef())
            && i == id
        {
            return Some(elem);
        }
        for child in get_children(elem)
        {
            if let Some(found) = find_by_id(child, id)
            {
                return Some(found);
            }
        }
        None
    }
}

unsafe fn click(elem: AXUIElementRef)
{
    let action = CFString::new(kAXPressAction);
    unsafe { AXUIElementPerformAction(elem, action.as_concrete_TypeRef()) };
}

fn retry<T, F>(max_sec: f64, mut f: F) -> Option<T>
where
    F: FnMut() -> Option<T>,
{
    let start = std::time::Instant::now();
    loop
    {
        if let Some(v) = f()
        {
            return Some(v);
        }
        if start.elapsed().as_secs_f64() >= max_sec
        {
            return None;
        }
        thread::sleep(Duration::from_millis(100));
    }
}
