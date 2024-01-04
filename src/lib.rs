use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use serde_json;
use zenoh_interface::{CmdVel, dual_shock_4::Axis};
use zenoh_manage_utils::param::get_str_param;
use zenoh_manage_utils::logger;

pub async fn wheel_controller(node_name:&str, yaml_path:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = get_str_param(yaml_path, node_name, "sub_topic", "joy".to_string());
    let pub_topic = get_str_param(yaml_path, node_name, "pub_topic", "wheel/cmd_vel".to_string());

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop
    {
        let sample = subscriber.recv_async().await.unwrap();
                
        let axis:Axis = serde_json::from_str(&sample.value.to_string()).unwrap();

        let send = CmdVel{
            x:axis.joy_left_x,
            y:axis.joy_left_y,
            rotation_power:axis.joy_right_x
        };

        let buf = serde_json::to_string(&send).unwrap();

        publisher.put(buf.clone()).res().await.unwrap();
    }
}