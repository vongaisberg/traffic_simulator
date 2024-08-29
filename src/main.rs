#![feature(float_gamma)]
#![feature(thread_sleep_until)]
pub mod packet_train;

use std::net::UdpSocket;
use std::{env, thread, time};

use packet_train::PacketTrain;
use rand_distr::{Distribution, Normal};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target_bandwidth = env::args()
        .nth(1)
        .expect("Please provide the target bandwidth in Mbit/s")
        .parse::<f64>()
        .expect("Please provide a valid number")
        * 1_000_000.
        / 8.;

        let target_ip = env::args().nth(2)
        .expect("Please provide the target IP address");

    let mut trains = [
        //Simulated Video Streaming (150ms of traffic every 5s)
        PacketTrain::new(
            5_000.,
            5.,
            Normal::new(150., 50.).unwrap(),
            Normal::new(1500., 50.).unwrap(),
            Normal::new(10_000_000., 5_000_000.).unwrap(),
        ),
        //Simulated rare big download (1s of traffic once in a while)
        PacketTrain::new(
            60_000.,
            0.5,
            Normal::new(2_000., 1_000.).unwrap(),
            Normal::new(1400., 50.).unwrap(),
            Normal::new(10_000_000., 5_000_000.).unwrap(),
        ),
        //Simulated noise (small packets often)
        PacketTrain::new(
            100.,
            0.5,
            Normal::new(10., 3.).unwrap(),
            Normal::new(200., 50.).unwrap(),
            Normal::new(10_000_000., 5_000_000.).unwrap(),
        ),
        // Bigger noise
        PacketTrain::new(
            100.,
            0.5,
            Normal::new(10., 3.).unwrap(),
            Normal::new(1400., 100.).unwrap(),
            Normal::new(10_000_000., 5_000_000.).unwrap(),
        ),
        // Really fast noise
        PacketTrain::new(
            10.,
            0.5,
            Normal::new(1., 0.3).unwrap(),
            Normal::new(100., 50.).unwrap(),
            Normal::new(1_000_000., 500_000.).unwrap(),
        ),
    ];

    //Sum of total mean bandwidth
    let mean_bandwidth = trains
        .iter()
        .map(|train: &PacketTrain| train.mean_bandwith())
        .sum::<f64>();
    println!(
        "Mean bandwidth: {:.3}Mbit/s",
        mean_bandwidth / 1_000_000. * 8.
    );
    let factor = target_bandwidth / mean_bandwidth;
    for train in trains.iter_mut() {
        train.bandwith_dist = Normal::new(
            train.bandwith_dist.mean() * factor,
            train.bandwith_dist.std_dev(),
        )
        .unwrap();
    }

    run(&socket, &mut trains, target_ip)?;

    Ok(())
}

fn run(socket: &UdpSocket, trains: &mut [PacketTrain], target_ip: String) -> std::io::Result<()> {
    let buf = [0; 1420]; // Wireguard MTU is 1420

    loop {
        let mut any_on = false;
        let earliest = trains
            .iter()
            .map(|train| train.current_status_until)
            .min()
            .unwrap();

        for train in trains.iter_mut() {
            let now = std::time::Instant::now();
            if now > train.current_status_until {
                train.flip_status();
                // println!("{:?}", output);
            }
            if train.currently_on {
                any_on = true;
                // If average bandwith is smaller than the expected bandwith
                if train.current_bandwith * train.current_status_since.elapsed().as_secs_f64()
                    > train.sum_of_data
                {
                    let data_to_send = train.size_dist.sample(&mut rand::thread_rng()) as usize;
                    let data_to_send = if data_to_send > buf.len() {
                        buf.len()
                    } else {
                        data_to_send
                    };
                    socket.send_to(&buf[..data_to_send], &target_ip)?;
                    // print!("Sent {} bytes\n", data_to_send);
                    train.sum_of_data += (data_to_send + 46) as f64; // 46 is the size of all of the headers
                } else {
                    // print!("Bandwith exceeded\n");
                }
            }
        }
        // Sleep if no packet train is active and the next packet train is not due for 1ms
        if !any_on && (earliest - std::time::Instant::now()).as_micros() > 1000 {
            // println!(
            //     "Sleeping for: {}",
            //     (earliest - std::time::Instant::now()).as_millis()
            // );
            thread::sleep_until(earliest)
        }
        // thread::sleep(Duration::from_micros(100));
    }
}
