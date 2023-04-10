use vumi_engine::run;

fn main() {
    pollster::block_on(run(
        "Vumi_Engine".to_string(),
        1024,768
    ));
}