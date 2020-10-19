use std::{
    cell::{
        Cell,
        RefCell,
    },
    rc::Rc,
};

use crate::deps::{
    bincode,
    holodeck_core::messages::{
        SimulationState,
        SpawnRequest,
    },
    holodeck_viewer::app::BackendChannel,
    js_sys,
    wasm_bindgen::{
        prelude::*,
        JsCast,
    },
    web_sys::{
        ErrorEvent,
        MessageEvent,
    },
};

use crate::deps::cfg_if::cfg_if;

cfg_if! {

    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


struct BackendChannelWrapper {
    tx: Rc<RefCell<Vec<SpawnRequest>>>,
    rx: Rc<Cell<Option<Box<SimulationState>>>>,
}


impl BackendChannel for BackendChannelWrapper {
    type Rx = Box<SimulationState>;
    type Tx = SpawnRequest;

    fn send(
        &self,
        value: Self::Tx,
    ) {
        self.tx.borrow_mut().push(value)
    }

    fn recv(&self) -> Option<Self::Rx> {
        self.rx.take()
    }
}


fn start_websocket(
    url: String
) -> Result<Box<dyn BackendChannel<Tx = SpawnRequest, Rx = Box<SimulationState>>>, JsValue> {
    // Connect to an echo server
    let ws = crate::deps::web_sys::WebSocket::new(&url)?;

    let backend_channel = BackendChannelWrapper {
        tx: Rc::new(RefCell::new(vec![])),
        rx: Rc::new(Cell::new(None)),
    };
    let frontend_tx = backend_channel.rx.clone();

    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    // create callback
    let counter = Rc::new(Cell::new(0usize));

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            let buf = array.to_vec();
            counter.set(counter.get() + 1);
            console_log!("Received message; bytes={}", len);

            let current_msg = frontend_tx.take();
            let msg = if let Some(_) = current_msg.as_ref() {
                // skip message deserializing and enqueuing message if the current message has not been
                // retrieved.
                current_msg
            } else {
                let sim_state = bincode::deserialize_from::<_, Box<SimulationState>>(&buf[..])
                    .map_err(|err| console_log!("ERROR: {:?}", err))
                    .ok();

                sim_state
            };

            frontend_tx.set(msg);
        // here you can for example use Serde Deserialize decode the message
        // for demo purposes we switch back to Blob-type and send off another binary message
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    //    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
        // match cloned_ws.send_with_str("ping") {
        //     Ok(_) => console_log!("message successfully sent"),
        //     Err(err) => console_log!("error sending message: {:?}", err),
        // }
        // // send off binary message
        // match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
        //     Ok(_) => console_log!("binary message successfully sent"),
        //     Err(err) => console_log!("error sending message: {:?}", err),
        // }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(Box::new(backend_channel))
}

#[wasm_bindgen]
pub fn run(url: JsValue) -> Result<(), JsValue> {
    crate::deps::console_error_panic_hook::set_once();
    let url = url
        .as_string()
        .map(|url| format!("ws://{}:5999", url))
        .unwrap_or("ws://localhost:5999".to_string());

    let backend_channel = start_websocket(url).unwrap();
    holodeck_viewer::app::PlayerGameClient::run(Some(backend_channel));
    Ok(())
}
