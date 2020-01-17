use std::convert::{TryInto};
use std::fmt;
use std::marker;
use std::mem;
use std::time::{Duration};
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::thread;

pub const MAXCHARLIFELESS: usize =  5;

pub trait FromBytes {
    fn from_bytes(buf: &[u8]) -> Self;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Packet<TData>
    where
        TData: Sized + Send + ToBytes + FromBytes + Default {
    
    pub sign: [u8;3], // "GDP"
    pub size: u16,
    pub data: TData
}

impl<TData> Packet<TData>
    where
        TData: Sized + Send + ToBytes + FromBytes + Default {

    pub fn new(msg: TData) -> Self {
        Self {
            sign: b"GDP".to_owned(),
            size: mem::size_of::<Self>() as u16,
            data: msg
        }        
    }
}

impl<TData> FromBytes for Packet<TData>
    where
        TData: Sized + Send + ToBytes + FromBytes + Default {

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sign: buf[0..3].try_into().unwrap(),
            size: u16::from_le_bytes(buf[3..5].try_into().unwrap()),
            data: TData::from_bytes(&buf[5..]),
        }
    }
}

impl<TData> ToBytes for Packet<TData>
    where
        TData: Sized + Send + ToBytes + FromBytes + Default {

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8;mem::size_of_val(self)];
        buf[..3].copy_from_slice(&self.sign);
        buf[3..5].copy_from_slice(&self.size.to_le_bytes());
        buf[5..113].copy_from_slice(&self.data.to_bytes());

        buf
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
#[repr(C)]
pub struct PacketLifelessInfo {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub d: i16,
    pub damage: i16,
}

#[derive(Default, Debug, PartialEq, Clone)]
#[repr(C)]
pub struct PacketCharInfo {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub a: i16,
    pub d: i16,
    pub dhit: i16,
    pub numchar: i16,
    pub idchar: i16,
    pub totchar: i16,
    pub totenemies: i16,
    pub exit: bool,
    pub healt: i16,
    pub stamina: i16,
    pub damage: i16,
    pub idmap: i16,
    pub totlifeless: i16,
	pub step: i16,
    pub vision: i16,
    pub listlifeless: [Option<PacketLifelessInfo>; MAXCHARLIFELESS],
}

impl FromBytes for PacketCharInfo {
    fn from_bytes(buf: &[u8]) -> Self {
        let mut lifeless: [Option<PacketLifelessInfo>; MAXCHARLIFELESS] = [None,None,None,None,None];

        for i in 0..MAXCHARLIFELESS {
            let j = 37 + i * mem::size_of::<PacketLifelessInfo>();

            let life = PacketLifelessInfo {
                x: i16::from_le_bytes(buf[j..j+2].try_into().unwrap()),
                y: i16::from_le_bytes(buf[j+2..j+4].try_into().unwrap()),
                w: i16::from_le_bytes(buf[j+4..j+6].try_into().unwrap()),
                h: i16::from_le_bytes(buf[j+6..j+8].try_into().unwrap()),
                d: i16::from_le_bytes(buf[j+8..j+10].try_into().unwrap()),
                damage: i16::from_le_bytes(buf[j+10..j+12].try_into().unwrap()),
            };

            if life != Default::default() {
                lifeless[i] = Some(life);
            }
        }

        Self {
            x: i16::from_le_bytes(buf[0..2].try_into().unwrap()),
            y: i16::from_le_bytes(buf[2..4].try_into().unwrap()),
            w: i16::from_le_bytes(buf[4..6].try_into().unwrap()),
            h: i16::from_le_bytes(buf[6..8].try_into().unwrap()),
            a: i16::from_le_bytes(buf[8..10].try_into().unwrap()),
            d: i16::from_le_bytes(buf[10..12].try_into().unwrap()),
            dhit: i16::from_le_bytes(buf[12..14].try_into().unwrap()),
            numchar: i16::from_le_bytes(buf[14..16].try_into().unwrap()),
            idchar: i16::from_le_bytes(buf[16..18].try_into().unwrap()),
            totchar: i16::from_le_bytes(buf[18..20].try_into().unwrap()),
            totenemies: i16::from_le_bytes(buf[20..22].try_into().unwrap()),
            exit: buf[22] == 1,
            healt: i16::from_le_bytes(buf[23..25].try_into().unwrap()),
            stamina: i16::from_le_bytes(buf[25..27].try_into().unwrap()),
            damage: i16::from_le_bytes(buf[27..29].try_into().unwrap()),
            idmap: i16::from_le_bytes(buf[29..31].try_into().unwrap()),
            totlifeless: i16::from_le_bytes(buf[31..33].try_into().unwrap()),
            step: i16::from_le_bytes(buf[33..35].try_into().unwrap()),
            vision: i16::from_le_bytes(buf[35..37].try_into().unwrap()),
            listlifeless: lifeless,
        }
    }
}

impl ToBytes for PacketCharInfo {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; mem::size_of_val(self)];

        buf[0..2].copy_from_slice(&self.x.to_le_bytes());
        buf[2..4].copy_from_slice(&self.y.to_le_bytes());
        buf[4..6].copy_from_slice(&self.w.to_le_bytes());
        buf[6..8].copy_from_slice(&self.h.to_le_bytes());
        buf[8..10].copy_from_slice(&self.a.to_le_bytes());
        buf[10..12].copy_from_slice(&self.d.to_le_bytes());
        buf[12..14].copy_from_slice(&self.dhit.to_le_bytes());
        buf[14..16].copy_from_slice(&self.numchar.to_le_bytes());
        buf[16..18].copy_from_slice(&self.idchar.to_le_bytes());
        buf[18..20].copy_from_slice(&self.totchar.to_le_bytes());
        buf[20..22].copy_from_slice(&self.totenemies.to_le_bytes());
        buf[22] = self.exit as u8;
        buf[23..25].copy_from_slice(&self.healt.to_le_bytes());
        buf[25..27].copy_from_slice(&self.stamina.to_le_bytes());
        buf[27..29].copy_from_slice(&self.damage.to_le_bytes());
        buf[29..31].copy_from_slice(&self.idmap.to_le_bytes());
        buf[31..33].copy_from_slice(&self.totlifeless.to_le_bytes());
        buf[33..35].copy_from_slice(&self.step.to_le_bytes());
        buf[35..37].copy_from_slice(&self.vision.to_le_bytes());

        for (i, lifeless) in self.listlifeless.iter().enumerate() {
            if let Some(lifeless) = lifeless {
                let j = 37 + i * mem::size_of_val(lifeless);
                buf[j..j+2].copy_from_slice(&lifeless.x.to_le_bytes());
                buf[j+2..j+4].copy_from_slice(&lifeless.y.to_le_bytes());
                buf[j+4..j+6].copy_from_slice(&lifeless.w.to_le_bytes());
                buf[j+6..j+8].copy_from_slice(&lifeless.h.to_le_bytes());
                buf[j+8..j+10].copy_from_slice(&lifeless.d.to_le_bytes());
                buf[j+10..j+12].copy_from_slice(&lifeless.damage.to_le_bytes());
            }
        }
    
        buf
    }
}

pub struct Client<TMsg: 'static + fmt::Debug + ToBytes + FromBytes + Send + Default> {
    stream: Option<TcpStream>,
    chan: Option<(Sender<TMsg>, Receiver<TMsg>)>
}

impl<TMsg> Client<TMsg>
    where
        TMsg: 'static + fmt::Debug + ToBytes + FromBytes + Send + Default {

    pub fn connect(address: &SocketAddr) -> Self {
        let stream = TcpStream::connect_timeout(address, Duration::from_secs(10)).expect("It must connect to server.");
        stream.set_nodelay(true).unwrap();
        stream.set_nonblocking(true).unwrap();

        Self {
            stream: Some(stream),
            chan: None,
        }
    }

    pub fn send(&self, msg: TMsg) {
        self.chan.as_ref().unwrap().0.send(msg).unwrap();
    }

    pub fn recv(&self) -> TMsg {
        self.chan.as_ref().unwrap().1.recv().unwrap()
    }

    pub fn start(&mut self) {
        let (tx, rx) = channel(); // send channel
        let (r_tx, r_rx) = channel(); // recv channel

        let mut stream = self.stream.take().unwrap();
        self.chan = Some((tx, r_rx));

        thread::spawn(move || {
            let mut buf = vec![0;std::mem::size_of::<Packet<TMsg>>()];

            loop {
                let mut done_something = false;
                
                // sender part
                match rx.try_recv() {
                    Ok(msg) => {
                        let packet = Packet::<TMsg> {
                            sign: b"GDP".to_owned(),
                            size: buf.len() as u16,
                            data: msg
                        };

                        let packet_bytes = packet.to_bytes();
                        stream.write_all(&packet_bytes).unwrap();

                        done_something = true;
                    },
                    Err(TryRecvError::Disconnected) => println!("Can't send message. Channel disconnected."),
                    Err(TryRecvError::Empty) => ()
                }

                // receiver part
                match stream.peek(&mut buf) {
                    Ok(size) if size >= buf.len() => {
                        stream.read_exact(&mut buf[0..size]).unwrap();

                        let packet = Packet::from_bytes(&buf[0..size].to_owned());
                        r_tx.send(packet.data).unwrap();
                        
                        done_something = true;
                    },
                    Ok(_) => (),
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
                    Err(e) => panic!(e)
                }

                if !done_something {
                    thread::yield_now();
                }
            }
        });
    }
}

#[derive(Debug)]
pub enum Message<TMsg>
    where 
        TMsg: 'static + fmt::Debug + ToBytes + FromBytes + Send + Default {
    Accepted(SocketAddr),
    Broadcast(TMsg),
    BroadcastExcept(TMsg, SocketAddr),
    Direct(TMsg, SocketAddr),
}

pub struct Server<TMsg>
    where
        TMsg: 'static + fmt::Debug + ToBytes + FromBytes + Send + Default {

    chan: Option<(Sender<Message<TMsg>>, Receiver<Message<TMsg>>)>,
}

impl<TMsg> Server<TMsg>
    where
        TMsg: 'static + fmt::Debug + ToBytes + FromBytes + Send + Default {

    pub fn new() -> Self {
        Self {
            chan: None
        }
    }

    pub fn send(&self, msg: Message<TMsg>) {
        if let Message::Accepted(_) = msg {
            panic!("This type of message is not allowed to be sent.");
        }

        self.chan.as_ref().unwrap().0.send(msg).unwrap();
    }

    pub fn recv(&self) -> Message<TMsg> {
        self.chan.as_ref().unwrap().1.recv().unwrap()
    }

    pub fn listen(&mut self, port: u16) {
        let (tx, rx) = channel(); // send channel
        let (r_tx, r_rx) = channel(); // receive channel
        self.chan = Some((tx, r_rx));

        thread::spawn(move || {
            let addr = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), port);
            let listener = TcpListener::bind(addr).unwrap();
            listener.set_nonblocking(true).unwrap();

            let mut clients = Vec::with_capacity(10);
            let mut buf = vec![0;std::mem::size_of::<Packet<TMsg>>()];

            loop {
                let mut done_something = false;

                // check new connections (accept)
                if let Ok((stream, addr)) = listener.accept() {
                    stream.set_nonblocking(true).unwrap();
                    stream.set_nodelay(true).unwrap();
                    clients.push((stream, addr));

                    r_tx.send(Message::Accepted(addr)).unwrap();

                    done_something = true;
                }

                // check send data channel
                // sender part
                match rx.try_recv() {
                    Ok(msg) => {
                        match msg {
                            Message::Broadcast(msg) => {
                                let packet = Packet::<TMsg>::new(msg);
                                let packet_bytes = packet.to_bytes();

                                clients
                                    .iter_mut()
                                    .map(|(c, _)| c)
                                    .for_each(|stream| {
                                        stream.write_all(&packet_bytes).unwrap();
                                    });
                            },
                            Message::BroadcastExcept(msg, client_addr) => {
                                let packet = Packet::<TMsg>::new(msg);
                                let packet_bytes = packet.to_bytes();

                                clients
                                    .iter_mut()
                                    .filter(|(_, addr)| *addr != client_addr)
                                    .map(|(c, _)| c)
                                    .for_each(|stream| {
                                        stream.write_all(&packet_bytes).unwrap();
                                    });
                            },
                            Message::Direct(msg, client_addr) => {
                                let stream = clients.iter_mut().find(|(_, addr)| *addr == client_addr).map(|(c, _)| c).unwrap();
                                let packet = Packet::<TMsg>::new(msg);
                                let packet_bytes = packet.to_bytes();
                                stream.write_all(&packet_bytes).unwrap();
                            },
                            Message::Accepted(_) => unreachable!(),
                        }

                        done_something = true;
                    },
                    Err(TryRecvError::Disconnected) => println!("Can't send message. Channel disconnected."),
                    Err(TryRecvError::Empty) => ()
                }

                // check recv TCP data and send
                for (stream, client_addr) in clients.iter_mut() {
                    match stream.peek(&mut buf) {
                        Ok(size) if size >= buf.len() => {
                            stream.read_exact(&mut buf[0..size]).unwrap();
    
                            let packet = Packet::<TMsg>::from_bytes(&buf[0..size].to_owned());
                            let msg = Message::Direct(packet.data, *client_addr);

                            r_tx.send(msg).unwrap();
                            
                            done_something = true;
                        },
                        Ok(_) => (),
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
                        Err(e) => panic!(e)
                    }
                }

                if !done_something {
                    thread::yield_now();
                }
            }
        });
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn usage_client() {
        println!("Tamanho do pacote: {}", std::mem::size_of::<PacketCharInfo>());
        thread::spawn(|| {
            let addr = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 34000);
            let listener = TcpListener::bind(addr).unwrap();
            //listener.set_nonblocking(true).unwrap();
            
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut buf = [0u8; 114];
                        let read = stream.read(&mut buf).unwrap();
                        let packet = Packet::<PacketCharInfo>::from_bytes(&buf);
                        println!("client->server ({}): {:?}", read, packet);
                        stream.write_all(&buf).unwrap();
                        break;
                    },
                    Err(e) => println!("Erro: {:?}", e)
                }
            }
        });

        let addr = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 34000);
        let mut client = Client::connect(&SocketAddr::V4(addr));
        client.start();

        client.send(PacketCharInfo {
            x: 1,
            y: 2,
            w: 3,
            h: 4,
            a: 5,
            d: 6,
            dhit: 7,
            numchar: 8,
            idchar: 9,
            totchar: 10,
            totenemies: 11,
            exit: true,
            healt: 13,
            stamina: 14,
            damage: 15,
            idmap: 16,
            totlifeless: 17,
            step: 18,
            vision: 19,
            listlifeless: [
                None, None, None, None,
                Some(PacketLifelessInfo {
                    x: 20,
                    y: 21,
                    w: 22,
                    h: 23,
                    d: 24,
                    damage: 25,
                })
            ]
        });
        thread::sleep(Duration::from_secs(5));
    }

    #[test]
    fn usage_client_server() {
        let mut server = Server::new();
        server.listen(34000);

        let addr = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 34000);
        let mut client = Client::connect(&SocketAddr::V4(addr));
        client.start();

        let packet = PacketCharInfo {
            x: 1,
            y: 2,
            w: 3,
            h: 4,
            a: 5,
            d: 6,
            dhit: 7,
            numchar: 8,
            idchar: 9,
            totchar: 10,
            totenemies: 11,
            exit: true,
            healt: 13,
            stamina: 14,
            damage: 15,
            idmap: 16,
            totlifeless: 17,
            step: 18,
            vision: 19,
            listlifeless: [
                None, None, None, None,
                Some(PacketLifelessInfo {
                    x: 20,
                    y: 21,
                    w: 22,
                    h: 23,
                    d: 24,
                    damage: 25,
                })
            ]
        };

        client.send(packet.clone());
        server.send(Message::Broadcast(packet.clone()));

        let packet2 = client.recv();
        assert_eq!(packet, packet2);

        let msg = server.recv();
        match msg {
            Message::Accepted(_) => (),
            _ => panic!("Unexpected type of message")
        }
        
        let msg = server.recv();
        match msg {
            Message::Direct(packet3, _) => assert_eq!(packet2, packet3),
            _ => panic!("Unexpected type of message")
        }
    }

    #[test]
    fn ser_des() {
        let packet = 
            Packet {
                sign: b"GDP".to_owned(),
                size: mem::size_of::<Packet<PacketCharInfo>>() as u16,
                data: PacketCharInfo {
                    x: 1,
                    y: 2,
                    w: 3,
                    h: 4,
                    a: 5,
                    d: 6,
                    dhit: 7,
                    numchar: 8,
                    idchar: 9,
                    totchar: 10,
                    totenemies: 11,
                    exit: true,
                    healt: 13,
                    stamina: 14,
                    damage: 15,
                    idmap: 16,
                    totlifeless: 17,
                    step: 18,
                    vision: 19,
                    listlifeless: [
                        None, None, None, None,
                        Some(PacketLifelessInfo {
                            x: 20,
                            y: 21,
                            w: 22,
                            h: 23,
                            d: 24,
                            damage: 25,
                        })
                    ]
                }
            };
        
        let buf = packet.to_bytes();
        let packet2 = Packet::<PacketCharInfo>::from_bytes(&buf);

        assert_eq!(packet.sign, packet2.sign);
        assert_eq!(packet.data,  packet2.data);
    }
}
