use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use encoding_rs_io::DecodeReaderBytesBuilder;

use std::time::{Duration, Instant};

mod ost;
use ost::Container;

fn pair_process(line: Vec<u8>) -> (u64, Vec<u8>) {
    let mut a = line.split(|&a| a == b' ');
    let first = a.next().unwrap();
    let second = a.next().unwrap();

    let index = String::from_utf8(first.to_vec())
        .unwrap()
        .parse::<u64>()
        .unwrap();

    (index, second.to_vec())
}

fn read_pairs<R, T, V>(reader: &mut R, proc: T) -> Vec<V>
where
    R: BufRead,
    T: Fn(Vec<u8>) -> V,
{
    reader
        .split(b'\n')
        .flatten()
        .map(|mut line| {
            if line.ends_with(b"\r") {
                line.pop();
            }
            line
        })
        .map(proc)
        .collect()
}

fn get_reader<P>(path: P) -> BufReader<encoding_rs_io::DecodeReaderBytes<File, Vec<u8>>>
where
    P: AsRef<Path> + Display,
{
    BufReader::new(
        DecodeReaderBytesBuilder::new()
            .build(File::open(&path).expect(&(format!("failed to open {} file", path)))),
    )
}

fn main() {
    println!("Start processing");

    let mut w_reader = get_reader("./test_samples/write.txt");
    let mut m_reader = get_reader("./test_samples/modify.txt");
    let mut r_reader = get_reader("./test_samples/read.txt");

    let wr_vec = read_pairs::<_, _, Vec<u8>>(&mut w_reader, |l| l);
    let mod_vec = read_pairs::<_, _, (u64, Vec<u8>)>(&mut m_reader, pair_process);
    let read_vec = read_pairs::<_, _, (u64, Vec<u8>)>(&mut r_reader, pair_process);

    let mut container = ost::OrderStatisticTree::new();
    wr_vec.into_iter().for_each(|s| container.insert(s));

    let res = mod_vec
        .iter()
        .zip(read_vec.iter())
        .map(|(modif, read)| {
            let start = Instant::now();

            container.remove(modif.0);
            container.insert(modif.1.clone());
            let v = container.get(read.0);

            let duration = start.elapsed();

            (read.1.clone() == v.unwrap(), duration)
        })
        .fold((true, Duration::new(0, 0)), |(acc, total_time), f| {
            (acc && f.0, total_time + f.1)
        });

    println!("result: {}. elapsed time {:?}", res.0, res.1);

    println!("End processing")
}
