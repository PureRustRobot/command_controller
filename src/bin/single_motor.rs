use async_std;
use command_controller::single_motor;
use zenoh::Error;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let controller_task = async_std::task::spawn(single_motor("single_motor", "game_con", "cmd/1", "up", "down"));

    controller_task.await?;

    Ok(())
}