use wry::Result;
use wry::{Application, Attributes};

fn main() -> Result<()> {
    let mut app = Application::new()?;

    let attributes = Attributes {
        transparent: false,
        debug: true,
        url: Some("https://www.wirple.com/".to_string()),

        //title: String::from("3D Render Test ^ ^"),
        fullscreen: true,
        //transparent: true, // <- Future
        //decorations: true,
        // maximized: true, and others from (https://docs.rs/wry/0.5.0/wry/struct.Attributes.html)
        ..Default::default()
    };

    app.add_window(attributes, None)?;
    app.run();
    Ok(())
}

// Test Result:
// CPU: i7 9750H || GPU: Intel(R) UHD Graphics 630
// Linux kernel 5.8.18-18-ibryza-standard-xin
// Mesa Mesa 20.2.6
// ================================================
// Canvas score - Test 1: 542 - Test 2: 368
// WebGL score - Test 1: 1390 - Test 2: 1342
// Total score: 3642
/* To Compare Brave 88.1.20.108
    Canvas score - Test 1: 606 - Test 2: 1809
    WebGL score - Test 1: 813 - Test 2: 819
    Total score: 4047
*/
