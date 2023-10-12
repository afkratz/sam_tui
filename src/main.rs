use noodles_bam as bam;
use noodles_core::Position;

use std::time::{Duration,Instant};

fn print_type<T>(_:&T){
    println!("{}",std::any::type_name::<T>())
}

fn main() {
    println!("Hello, world!");
    let mut reader = bam::reader::Builder::default().build_from_path(
        "../igm-storage2.ucsd.edu/r64278e_20230307_232555/1_A01/m64278e_230307_234313.reads.bam"
        ).expect("Failed at build_from_path");
    
    let header = reader.read_header().expect("Failed at read_header");
    let start = Instant::now();
    
    let mut loop_count:i32 = 0;
    for result in reader.records(&header){
        loop_count+=1;
        //if loop_count%10000==0{println!("{}",loop_count)};
        let record = result.expect("Failed at result.expect");
        //print_type(&record);
        //let l = record.sequence().len();
        let string_ver = record.sequence().to_string();
        //println!("Rec len: {}, String len: {}",l,string_ver.len()); 
        if loop_count==1_000_000{break}
    }
    println!("Finished exec {} items in {} millis\n,average time elapse:{}", loop_count,start.elapsed().as_millis(),(start.elapsed().as_millis() as f32 /loop_count as  f32))
}
