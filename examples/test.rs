use cronet_rs::{
    Buffer, BufferCallback, CronetError, Destroy, Engine, EngineParams, Executor,
    UrlRequest, UrlRequestCallback, UrlRequestParams, UrlResponseInfo,
    UrlRequestCallbackHandler
};

struct RequestCallbackHandler {
    body: Vec<u8>
}

impl RequestCallbackHandler {
    pub fn new() -> Self {
        Self {
            body: Vec::new()
        }
    }
}

impl UrlRequestCallbackHandler for RequestCallbackHandler {
    fn on_redirect_received(
        &self,
        _: UrlRequestCallback,
        request: UrlRequest,
        _: UrlResponseInfo,
        _: &str,
    ) {
        println!("on_redirect_received");
        request.follow_redirect();
    }

    fn on_response_started(&self, _: UrlRequestCallback, request: UrlRequest, _: UrlResponseInfo) {
        println!("Response started, reading into buffer");

        let data = Box::new([0; 16000]);
        let len = data.len() as u64;

        let buffer = Buffer::new_with_data_and_callback(data, len, BufferCallback::new(|self_, buffer| {
            println!("Callback!");
            self_.destroy();
        }));

        request.read(buffer);
    }

    fn on_read_completed(
        &mut self,
        _: UrlRequestCallback,
        request: UrlRequest,
        _: UrlResponseInfo,
        buffer: Buffer,
        _: u64,
    ) {
        println!("on_read_completed");

        // This really doesn't seem ideal.
        let data: Box<[u8; 16000]> = buffer.data();
        self.body.extend_from_slice(&*data);
        buffer.destroy();
        
        let data = Box::new([0; 16000]);
        let len = data.len() as u64;

        let buffer = Buffer::new_with_data_and_callback(data, len, BufferCallback::new(|self_, buffer| {
            println!("Callback!");
            self_.destroy();
        }));

        request.read(buffer);
    }

    fn on_succeeded(&self, _: UrlRequestCallback, _: UrlRequest, info: UrlResponseInfo) {
        println!("Success");
        println!("{}", info.url());
        println!("{}", info.status_code());
        println!("{}", info.status_text());

        let body = String::from_utf8_lossy(&self.body);
        println!("{}", body);
    }

    fn on_failed(
        &self,
        _: UrlRequestCallback,
        _: UrlRequest,
        _: UrlResponseInfo,
        _: CronetError,
    ) {
        println!("on_failed");
    }

    fn on_canceled(&self, _: UrlRequestCallback, _: UrlRequest, _: UrlResponseInfo) {
        println!("on_canceled");
    }
}

fn main() {
    let engine = Engine::new();
    let result = engine.start(EngineParams::new());
    println!("Engine start: {:?}", result);

    let request = UrlRequest::new();
    let params = UrlRequestParams::new();
    let callback = UrlRequestCallback::new(RequestCallbackHandler::new());

    let executor = Executor::new(|exec, runnable| {
        println!("In here");
        runnable.run();
    });

    let result = request.init_with_params(engine, "https://rymc.io/", params, callback, executor);
    println!("Request init: {:?}", result);

    let result = request.start();
    println!("Request start: {:?}", result);

    loop { 
        std::thread::sleep_ms(1000);
        println!("Waiting...");
    }
}
