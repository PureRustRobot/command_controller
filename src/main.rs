use zenoh::{
    config::Config,
    prelude::r#async::*,
};

use futures::select;
use serde_json;
use zenoh_interface::{CmdVel, dual_shock_4::Axis};

#[async_std::main]
async fn main(){
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = "/joy".to_string();
    let pub_topic = "/cmd_vel".to_string();

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();

    loop {
        select! (
            sample = subscriber.recv_async() =>{
                let sample = sample.unwrap();
                
                let axis:Axis = serde_json::from_str(&sample.value.to_string()).unwrap();

                let send = CmdVel{
                    x:axis.joy_left_x,
                    y:axis.joy_left_y,
                    rotation_power:axis.joy_right_x
                };

                let buf = serde_json::to_string(&send).unwrap();

                publisher.put(buf.clone()).res().await.unwrap();
            },
        );
    }
}