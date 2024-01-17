use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use prr_msgs::msg::*;
use zenoh_manage_utils::logger;

pub async fn joy_to_cmd_vel(
    node_name:&str, 
    sub_topic:&str,
    pub_topic:&str,

)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(pub_topic).res().await.unwrap();

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop
    {
        let sample = subscriber.recv_async().await.unwrap();
                
        let get_data = deserialize_joystick(sample.value.to_string());

        let cmd_vel = CmdVel{
            x:get_data.left_x,
            y:get_data.left_y,
            rotation_power:get_data.right_x
        };

        let serialized = serialize_cmdvel(&cmd_vel);

        logger::log_info(node_name, format!("Send:{}", serialized));

        publisher.put(serialized).res().await.unwrap();
    }
}

pub async fn button_to_single_motor(
    node_name:&str,
    sub_topic:&str,
    pub_topic:&str,
    positive_name:&str,
    negative_name:&str,
)
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(pub_topic).res().await.unwrap();

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop {
        let sample = subscriber.recv_async().await.unwrap();

        let get_data = deserialize_buttons(sample.value.to_string());

        let mut send_data = SingleMotor{
            power:0.0
        };

        if name_to_button(positive_name, &get_data) == 1.0
        {
            send_data.power = 1.0;
        }
        else if name_to_button(negative_name, &get_data) == 1.0{
            send_data.power = -1.0;
        }
    }
}

fn name_to_button(name:&str, btns:&Buttons)->f32
{
    match name {
        "circle"=>btns.circle,
        "cross"=>btns.cross,
        "cube"=>btns.cube,
        "triangle"=>btns.triangle,
        "up"=>btns.up_key,
        "down"=>btns.down_key,
        "left"=>btns.left_key,
        "right"=>btns.right_key,
        "r1"=>btns.r1,
        "l1"=>btns.l1,
        "r2"=>btns.r2,
        "l2"=>btns.l2,
        _=>{
            logger::log_error("button_to_value", "Failed to get button value".to_string());
            return 0.0
        }
    }
}
