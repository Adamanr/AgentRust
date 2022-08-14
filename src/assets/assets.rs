pub mod assets{
    use std::net::{IpAddr, Ipv4Addr};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio;
    use tokio::net::{TcpStream};
    use crate::assets::tun::tun::tun::create_tun;
    use crate::common::netutil::netutil::netutil::get_ipv4;
    use crate::Config;
    use std::net::Ipv4Addr;
    use std::os::unix::io::AsRawFd;
    use tokio_tun::{Tun, TunBuilder};

    pub async fn start_client(config: Config) {
        let tuns = create_tun(&config);
        let mut buf0 = [0u8; 1500];
        let (mut reader, mut writer) = tokio::io::split(tuns);
        loop {
            let n = reader.read(&mut buf0).await.expect("Error read");
            let b = &buf0[..n];
            println!("reading {} bytes from tuns: {:?}", buf0.len(), b);
            let ipv4 = IpAddr::V4(Ipv4Addr::new(b[0],b[1],b[2],b[3]));
            println!("IPv4 {}",ipv4);
            if !IpAddr::is_ipv4(&ipv4){
                continue
            }
            let (src_ipv4,dst_ipv4) = get_ipv4();
            if src_ipv4 == " " || dst_ipv4 == " " {
                continue
            }
            let addr = config.server_addr;
            println!("########8");
            tokio::spawn(async move {
                println!("#########9");
                let mut buffer = [0u8; 1500];
                loop {
                    let mut stream = TcpStream::connect(addr.as_str()).await.expect("Error stream");
                    println!("##########10");
                    let n = stream.read(&mut buffer).await.expect("Error read");
                    buffer.map(|e|{
                        print!("{}",e.to_string());
                    });
                    println!("\n###########11");
                    let b = &buffer[..n];
                    println!("{:?}",std::str::from_utf8(b));
                    println!("############12");
                    writer.write(&b).await.expect("");
                }
            }).await.expect("Error spawn");
            break
        }
    }


    pub fn create_tun(_config: &Config) -> Tun{
        let tuns = TunBuilder::new()
            .name("")
            .tap(false)
            .packet_info(false)
            .mtu(1350)
            .up()
            .address(Ipv4Addr::new(0, 0, 0, 0))
            .destination(Ipv4Addr::new(0, 0, 0, 0))
            .broadcast(Ipv4Addr::BROADCAST)
            .netmask(Ipv4Addr::new(255, 255, 255, 0))
            .try_build()
            .expect("Error tun");
        println!(
            "┌ name: {}\n├ fd: {} \n├ mtu: {}\n├ flags: {}\n├ address: {}\n├ destination: {}\n├ broadcast: {}\n└ netmask: {}",
            tuns.name(),
            tuns.as_raw_fd(),
            tuns.mtu().unwrap(),
            tuns.flags().unwrap(),
            tuns.address().unwrap(),
            tuns.destination().unwrap(),
            tuns.broadcast().unwrap(),
            tuns.netmask().unwrap(),
        );
        return tuns
    }

}