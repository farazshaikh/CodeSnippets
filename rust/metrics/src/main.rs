use metrics::LibObject;
fn main() {
    let registry = prometheus::default_registry();

    let lib_object = LibObject::new(&registry);
    lib_object.callfn();
    lib_object.callfn();
    lib_object.callfn();
    println!("{}", lib_object.get_call_count());
    println!("Hello, world!");

    let jh = std::thread::spawn(move || loop {
        lib_object.callfn();
    });
    prometheus_exporter::start("0.0.0.0:9184".parse().expect("failed to parse binding"))
        .expect("failed to start prometheus exporter");

    jh.join();
}
