use wayland_client::{
    protocol::{wl_buffer, wl_compositor, wl_output, wl_registry, wl_surface},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols_wlr::screencopy::v1::client::{
    zwlr_screencopy_frame_v1::{self, ZwlrScreencopyFrameV1},
    zwlr_screencopy_manager_v1::{self, ZwlrScreencopyManagerV1},
};

#[derive(Debug)]
struct AppData {
    screencpy_manager: Option<ZwlrScreencopyManagerV1>,
    output: Option<wl_output::WlOutput>,
    compositor: Option<wl_compositor::WlCompositor>,
    buffer: Option<wl_buffer::WlBuffer>,
}

impl AppData {
    fn new() -> Self {
        Self {
            screencpy_manager: None,
            output: None,
            compositor: None,
            buffer: None,
        }
    }

    fn is_ready(&self) -> bool {
        self.screencpy_manager.is_some()
            && self.output.is_some()
            && self.compositor.is_some()
            && self.buffer.is_some()
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            //println!("[{}] {} (v{})", name, interface, version);        // DEBUG
            if interface == "zwlr_screencopy_manager_v1" {
                state.screencpy_manager =
                    Some(registry.bind::<ZwlrScreencopyManagerV1, _, _>(name, version, qh, ()));
            } else if interface == "wl_output" {
                state.output =
                    Some(registry.bind::<wl_output::WlOutput, _, _>(name, version, qh, ()));
            } else if interface == "wl_compositor" {
                state.compositor =
                    Some(registry.bind::<wl_compositor::WlCompositor, _, _>(name, version, qh, ()));
            }
        }
    }
}

impl Dispatch<ZwlrScreencopyManagerV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &ZwlrScreencopyManagerV1,
        event: zwlr_screencopy_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got screencopy manager: {:?}", event);
    }
}

impl Dispatch<ZwlrScreencopyFrameV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &ZwlrScreencopyFrameV1,
        event: zwlr_screencopy_frame_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got screencopy frame: {:?}", event); // DEBUG
    }
}

impl Dispatch<wl_output::WlOutput, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_output::WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got output: {:?}", event);
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_compositor::WlCompositor,
        event: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got compositor: {:?}", event);
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_surface::WlSurface,
        event: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got surface: {:?}", event);
    }
}

// The main function of our program
fn main() {
    let mut state = AppData::new();

    let conn = Connection::connect_to_env().unwrap();
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());

    println!("Advertized globals:");

    event_queue.roundtrip(&mut state).unwrap();
    println!();

    if state.is_ready() {
        //dbg!(state);     TODO: debug
        state.screencpy_manager.unwrap().capture_output(
            0,
            &state.output.unwrap(),
            &qh,
            (),
        );
        let surface = state.compositor.unwrap().create_surface(&qh, ());
        loop {
            // TODO: attach and commit surface
            //surface.attach(Some(&state.buffer.unwrap()), 0, 0);

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    } else {
        println!("Not ready");
    }
}
