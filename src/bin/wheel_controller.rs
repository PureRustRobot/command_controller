use async_std;
use command_controller::joy_to_cmd_vel;
use zenoh::Error;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let w_controller_task = async_std::task::spawn(joy_to_cmd_vel("joy_to_wheel", "joy", "cmd/wheel"));

    w_controller_task.await?;

    Ok(())
}