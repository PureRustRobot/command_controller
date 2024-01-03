use async_std;
use command_controller::wheel_controller;
use zenoh::Error;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let w_controller_task = async_std::task::spawn(wheel_controller("./param/wheel_controller.yaml"));

    w_controller_task.await?;

    Ok(())
}