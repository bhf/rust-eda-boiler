use crate::model::order::Order;
use crate::repositories::order_repo::in_memory_order_repository::InMemoryOrderRepository;
use crate::services::oms::OMSService;
use crate::services::oms_service::oms_handler::OmsHandler;
use crate::services::oms_service::oms_handler_error::OmsHandlerError;
use disruptor::*;
use rusteron_client::{
    Aeron, AeronContext, AeronFragmentHandlerCallback, AeronHeader, Handler, Handlers, IntoCString,
};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod model;
mod repositories;
mod services;

fn main() {
    env_logger::init();

    let use_aeron = std::env::var("USE_AERON")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    log::info!("Starting up Application");

    let order_repository = Box::new(InMemoryOrderRepository::new());
    let oms_error = OmsHandlerError::new(1);
    let oms_handler = OmsHandler::new(oms_error, order_repository);

    if use_aeron {
        subscribe_to_aeron("aeron:ipc".parse().unwrap(), 808);
    } else {
        publish_messages(oms_handler);
    }
}

///
/// Subscribe to an Aeron subscription and publish
/// messages to the Ringbuffer to be processed by the OMS Handler.
///
fn subscribe_to_aeron(subscription_channel: String, stream_id: i32) {
    let order_repository = Box::new(InMemoryOrderRepository::new());
    let oms_error = OmsHandlerError::new(1);
    let mut oms_handler = OmsHandler::new(oms_error, order_repository);

    let event_factory = || Order::new(0, 0, 0);

    let event_handler = move |e: &Order, sequence: Sequence, _end_of_batch: bool| {
        log::info!("Received event with sequence: {}, Id: {}", sequence, e.id());
        let order = Order::new(e.id(), e.amount(), e.instrument_id());

        match oms_handler.process_order(order) {
            Ok(value) => {
                log::info!("Processed order: {}", e.id());
            }
            Err(error) => {
                log::error!("Error processing order: {}", error);
            }
        }
    };

    let builder = build_single_producer(1 << 8, event_factory, BusySpin);
    let rb = builder.handle_events_with(event_handler).build();

    log::info!("Subscribing to Aeron stream");

    // Start embedded media driver
    start_media_driver().unwrap();

    let aeron_context = AeronContext::new().unwrap();
    let aeron = Aeron::new(&aeron_context).unwrap();

    struct FragmentHandler {
        rb: SingleProducer<Order, SingleConsumerBarrier>,
    }
    impl AeronFragmentHandlerCallback for FragmentHandler {
        fn handle_aeron_fragment_handler(&mut self, buffer: &[u8], header: AeronHeader) -> () {
            // Decode from the buffer and publish to the Ringbuffer
            self.rb.publish(|e| {
                let count = u64::from_le_bytes(buffer[0..8].try_into().unwrap());
                let id = u64::from_le_bytes(buffer[8..16].try_into().unwrap());
                let amount = u64::from_le_bytes(buffer[16..24].try_into().unwrap());
                let instrument_id = u64::from_le_bytes(buffer[24..32].try_into().unwrap());
                e.populate(id, amount, instrument_id);
            });
        }
    }

    let (_assembler, fragment_handler) =
        Handler::leak_with_fragment_assembler(FragmentHandler { rb }).unwrap();

    let live_subscription = aeron
        .add_subscription(
            &*subscription_channel.into_c_string(),
            stream_id,
            Handlers::no_available_image_handler(),
            Handlers::no_unavailable_image_handler(),
            Duration::from_millis(100),
        )
        .ok();

    let sub = live_subscription.unwrap();

    thread::scope(|s| {
        s.spawn(move || loop {
            let _ = sub.poll(Some(&fragment_handler), 1000);
        });
    });
}

///
/// Start an embedded media driver.
///
pub fn start_media_driver() -> Result<(), Box<dyn std::error::Error>> {
    let aeron_context = rusteron_media_driver::AeronDriverContext::new()?;
    let aeron_driver = rusteron_media_driver::AeronDriver::new(&aeron_context)?;
    aeron_driver.start(true)?;
    log::info!("Aeron media driver started successfully.");

    aeron_driver.conductor().context().print_configuration();
    aeron_driver.main_do_work()?;
    log::info!("aeron dir: {:?}", aeron_context.get_dir());

    loop {
        aeron_driver.main_idle_strategy(aeron_driver.main_do_work()?);
    }
}

///
/// Publish messages onto the Ringbuffer.
///
fn publish_messages(mut oms_handler: OmsHandler) {

    let event_factory = || Order::new(0, 0, 0);

    let event_handler = move |e: &Order, sequence: Sequence, _end_of_batch: bool| {
        log::info!("Received event with sequence: {}, Id: {}", sequence, e.id());
        let order = Order::new(e.id(), e.amount(), e.instrument_id());

        match oms_handler.process_order(order) {
            Ok(value) => {
                log::info!("Processed order: {}", e.id());
            }
            Err(error) => {
                log::error!("Error processing order: {}", error);
            }
        }
    };

    let builder = build_single_producer(1 << 8, event_factory, BusySpin);
    let mut rb = builder.handle_events_with(event_handler).build();

    thread::scope(|s| {
        s.spawn(move || {
            let mut count: u64 = 0;

            loop {
                count += 1;

                rb.publish(|e| {
                    let id = count;
                    let amount = count * 2;
                    let instrument_id = if id % 2 == 0 { 0 } else { id + 1 };
                    e.populate(id, amount, instrument_id);
                });

                log::info!("Total events published to ringbuffer: {}", count);
                sleep(Duration::from_secs(1));
            }
        });
    });
}
