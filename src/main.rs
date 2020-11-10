use async_std::fs::OpenOptions;
use async_std::io::prelude::*;

use async_std::fs::File;
use async_std::io::SeekFrom;

use async_std::sync::{channel, Sender};
use async_std::task;

use futures::{future::{FutureExt}, select};
use std::path::PathBuf;

use glob::glob;

#[async_std::main]
async fn main() {
    let mut magn_input = PathBuf::new();
    let mut acc_input = PathBuf::new();
    let mut gyro_input = PathBuf::new();

    for entry in glob("/sys/bus/iio/devices/*/name").unwrap() {
        let p = entry.unwrap();

        let mut name_file = OpenOptions::new()
            .read(true)
            .open(&p)
            .await.unwrap();

        let mut contents = String::new();
        name_file.read_to_string(&mut contents).await.unwrap();

        let parent = p.parent().unwrap().to_path_buf();
        println!("{:?}", contents);
        match contents.trim() {
            "lis3mdl" => magn_input = parent,
            "lsm6ds3_accel" => acc_input = parent,
            "lsm6ds3_gyro" => gyro_input = parent,
            _ => ()
        }
    }

    if !magn_input.exists() {
        print!("missing input mag");
    }

    if !acc_input.exists() {
        print!("missing input acc");
    }

    if !gyro_input.exists() {
        print!("missing input gyro");
    }

    let magn_x = OpenOptions::new()
        .read(true)
        .open(magn_input.join("in_magn_x_raw"))
        .await.unwrap();

    let magn_y = OpenOptions::new()
        .read(true)
        .open(magn_input.join("in_magn_y_raw"))
        .await.unwrap();

    let magn_z = OpenOptions::new()
        .read(true)
        .open(magn_input.join("in_magn_z_raw"))
        .await.unwrap();

    let acc_x = OpenOptions::new()
        .read(true)
        .open(acc_input.join("in_accel_x_raw"))
        .await.unwrap();

    let acc_y = OpenOptions::new()
        .read(true)
        .open(acc_input.join("in_accel_y_raw"))
        .await.unwrap();

    let acc_z = OpenOptions::new()
        .read(true)
        .open(acc_input.join("in_accel_z_raw"))
        .await.unwrap();

    let gyro_x = OpenOptions::new()
        .read(true)
        .open(gyro_input.join("in_anglvel_x_raw"))
        .await.unwrap();

    let gyro_y = OpenOptions::new()
        .read(true)
        .open(gyro_input.join("in_anglvel_y_raw"))
        .await.unwrap();

    let gyro_z = OpenOptions::new()
        .read(true)
        .open(gyro_input.join("in_anglvel_z_raw"))
        .await.unwrap();

    let (magn_sender_x, magn_recv_x) = channel::<String>(1);
    let (magn_sender_y, magn_recv_y) = channel::<String>(1);
    let (magn_sender_z, magn_recv_z) = channel::<String>(1);

    let (acc_sender_x, acc_recv_x) = channel::<String>(1);
    let (acc_sender_y, acc_recv_y) = channel::<String>(1);
    let (acc_sender_z, acc_recv_z) = channel::<String>(1);

    let (gyro_sender_x, gyro_recv_x) = channel::<String>(1);
    let (gyro_sender_y, gyro_recv_y) = channel::<String>(1);
    let (gyro_sender_z, gyro_recv_z) = channel::<String>(1);

    read_raw_channel(magn_x, magn_sender_x).await;
    read_raw_channel(magn_y, magn_sender_y).await;
    read_raw_channel(magn_z, magn_sender_z).await;

    read_raw_channel(acc_x, acc_sender_x).await;
    read_raw_channel(acc_y, acc_sender_y).await;
    read_raw_channel(acc_z, acc_sender_z).await;

    read_raw_channel(gyro_x, gyro_sender_x).await;
    read_raw_channel(gyro_y, gyro_sender_y).await;
    read_raw_channel(gyro_z, gyro_sender_z).await;


    task::spawn(async move {
        let mut magn_raw_x = String::new();
        let mut magn_raw_y = String::new();
        let mut magn_raw_z = String::new();

        let mut acc_raw_x = String::new();
        let mut acc_raw_y = String::new();
        let mut acc_raw_z = String::new();

        let mut gyro_raw_x = String::new();
        let mut gyro_raw_y = String::new();
        let mut gyro_raw_z = String::new();

        loop {

            select! {
                x = magn_recv_x.recv().fuse() => magn_raw_x = x.unwrap(),
                y = magn_recv_y.recv().fuse() => magn_raw_y = y.unwrap(),
                z = magn_recv_z.recv().fuse() => magn_raw_z = z.unwrap(),

                x = acc_recv_x.recv().fuse() => acc_raw_x = x.unwrap(),
                y = acc_recv_y.recv().fuse() => acc_raw_y = y.unwrap(),
                z = acc_recv_z.recv().fuse() => acc_raw_z = z.unwrap(),

                x = gyro_recv_x.recv().fuse() => gyro_raw_x = x.unwrap(),
                y = gyro_recv_y.recv().fuse() => gyro_raw_y = y.unwrap(),
                z = gyro_recv_z.recv().fuse() => gyro_raw_z = z.unwrap(),
            };

            println!(
                "{0: >8}{1: >8}{2: >8}{3: >8}{4: >8}{5: >8}{6: >8}{7: >8}{8: >8}",

                magn_raw_x.trim(),
                magn_raw_y.trim(),
                magn_raw_z.trim(),

                acc_raw_x.trim(),
                acc_raw_y.trim(),
                acc_raw_z.trim(),

                gyro_raw_x.trim(),
                gyro_raw_y.trim(),
                gyro_raw_z.trim(),
            );
        }
    }).await;
}

async fn read_raw_channel(mut f: File, sender: Sender<String>) {
    task::spawn(async move {
        loop {
            let mut contents = String::new();
            f.read_to_string(&mut contents).await.unwrap();
            sender.send(contents).await;
            f.seek(SeekFrom::Start(0)).await.unwrap();
        }
    });
}
