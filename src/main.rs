fn main() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };
    let user_agent_string = "Mozilla/5.0 (SMART-TV; LINUX; Tizen 6.5) AppleWebKit/537.36 (KHTML, like Gecko) 85.0.4183.93/6.5 TV Safari/537.36".to_string();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Youtube TV")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_user_agent(&user_agent_string)
        .with_url("https://www.youtube.com/tv#/")?
        .with_initialization_script(
            r#"
			const orgAddEventListener = window.document.addEventListener;
			function hookedAddEventListener(eventType, ...args) {
				if (eventType === 'visibilitychange' || eventType === 'webkitvisibilitychange') {
					console.log(`Prevented youtube from registering ${eventType} event listener`);
				} else {
					orgAddEventListener(eventType, ...args);
				}
			}
			window.document.addEventListener = hookedAddEventListener;
			"#,
        )
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Youtube TV is now running!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
