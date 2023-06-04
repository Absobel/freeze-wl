use std::os::fd::AsRawFd;

use wayland_client::{
    protocol::{wl_buffer::{self, WlBuffer}, wl_compositor, wl_output, wl_registry, wl_shm, wl_shm_pool, wl_surface},
    Connection, Dispatch, QueueHandle, WEnum,
};
use wayland_protocols_wlr::screencopy::v1::client::{
    zwlr_screencopy_frame_v1::{self, ZwlrScreencopyFrameV1},
    zwlr_screencopy_manager_v1::{self, ZwlrScreencopyManagerV1},
};
use memmap2::MmapMut;

#[derive(Debug)]
struct BufferData {
    format: wl_shm::Format,
    width: u32,
    height: u32,
    stride: u32,
}

impl BufferData {
    fn new(format: wl_shm::Format, width: u32, height: u32, stride: u32) -> Self {
        Self {
            format,
            width,
            height,
            stride,
        }
    }

    fn size(&self) -> i32 {
        self.stride as i32 * self.height as i32
    }

    fn buffer(&self, shm_pool: &wl_shm_pool::WlShmPool, qh: &QueueHandle<AppData>) -> WlBuffer {
        shm_pool.create_buffer(
            0,
            self.width as i32,
            self.height as i32,
            self.stride as i32,
            self.format,
            qh,
            (),
        )
        
    }
}

#[derive(Debug)]
struct AppData {
    screencpy_manager: Option<ZwlrScreencopyManagerV1>,
    output: Option<wl_output::WlOutput>,
    compositor: Option<wl_compositor::WlCompositor>,
    shm: Option<wl_shm::WlShm>,

    buffer_data: Option<BufferData>,
}
impl AppData {
    fn new() -> Self {
        Self {
            screencpy_manager: None,
            output: None,
            compositor: None,
            shm: None,

            buffer_data: None,
        }
    }

    fn is_ready(&self) -> bool {
        self.screencpy_manager.is_some()
            && self.output.is_some()
            && self.compositor.is_some()
            && self.shm.is_some()
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
            } else if interface == "wl_shm" {
                state.shm = Some(registry.bind::<wl_shm::WlShm, _, _>(name, version, qh, ()));
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
        println!("Got screencopy manager: {:?}", event); // DEBUG
    }
}

impl Dispatch<ZwlrScreencopyFrameV1, ()> for AppData {
    fn event(
        state: &mut Self,
        _: &ZwlrScreencopyFrameV1,
        event: zwlr_screencopy_frame_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got screencopy frame: {:?}", event); // DEBUG

        match event {
            zwlr_screencopy_frame_v1::Event::Buffer {
                format,
                width,
                height,
                stride,
            } => {
                state.buffer_data = Some(BufferData::new(format.into_result().unwrap(), width, height, stride));
            }
            _ => {}
        }
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
        println!("Got output: {:?}", event); // DEBUG
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
        println!("Got compositor: {:?}", event); // DEBUG
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
        println!("Got surface: {:?}", event); // DEBUG
    }
}

impl Dispatch<wl_shm::WlShm, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_shm::WlShm,
        event: wl_shm::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got shm: {:?}", event); // DEBUG
    }
}

impl Dispatch<wl_shm_pool::WlShmPool, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_shm_pool::WlShmPool,
        event: wl_shm_pool::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got shm pool: {:?}", event); // DEBUG
    }
}

impl Dispatch<wl_buffer::WlBuffer, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_buffer::WlBuffer,
        event: wl_buffer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        println!("Got buffer: {:?}", event); // DEBUG
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

    let tmp_file = tempfile::tempfile().unwrap();

    event_queue.roundtrip(&mut state).unwrap();  // So that the state is ready

    if state.is_ready() {
        let frame = state.screencpy_manager.as_ref().unwrap().capture_output(
            0,
            state.output.as_ref().unwrap(),
            &qh,
            (),
        );
        event_queue.roundtrip(&mut state).unwrap();  // Have to take the events to create bufferdata
        

        let surface = state.compositor.as_ref().unwrap().create_surface(&qh, ());
                
        let shm_pool = state.shm.as_ref().unwrap().create_pool(
            tmp_file.as_raw_fd(),
            state.buffer_data.as_ref().unwrap().size(),
            &qh,
            (),
        );

        // Map the file into memory
        let mut mmap = unsafe { MmapMut::map_mut(&tmp_file).unwrap() };

        // Fill the buffer with white pixels
        let pixel = 0xFFFFFFFFu32;  // white in ARGB8888
        for i in (0..mmap.len()).step_by(4) {
            mmap[i..i+4].copy_from_slice(&pixel.to_ne_bytes());
        }

        let buffer = state.buffer_data.as_ref().unwrap().buffer(&shm_pool, &qh);
        surface.attach(Some(&buffer), 0, 0);

        event_queue.roundtrip(&mut state).unwrap();

        loop {
            // TODO: attach and commit surface
            surface.commit();

            event_queue.dispatch_pending(&mut state).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    } else {
        println!("Not ready");
    }
}
