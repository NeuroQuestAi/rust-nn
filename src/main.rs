use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Device};

const IMAGE_DIM: i64 = 784;
const HIDDEN_NODES: i64 = 128;
const LABELS: i64 = 10;

fn net(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::linear(vs / "layer1", IMAGE_DIM, HIDDEN_NODES, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs, HIDDEN_NODES, LABELS, Default::default()))
}

pub fn run() -> Result<()> {
    let m = tch::vision::mnist::load_dir("data")?;
    let vs = nn::VarStore::new(Device::Cpu);
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
    for epoch in 1..=100 {
        let loss = net.forward(&m.train_images).cross_entropy_for_logits(&m.train_labels);
        opt.backward_step(&loss);
        let test_accuracy = net.forward(&m.test_images).accuracy_for_logits(&m.test_labels);

        let loss_scalar: f64 = loss.double_value(&[]);
        let test_accuracy_scalar: f64 = test_accuracy.double_value(&[]);

        println!(
            "Epochs: {:4} train loss: {:8.5} test acc: {:5.2}%",
            epoch, loss_scalar, 100. * test_accuracy_scalar,
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    run()
}

