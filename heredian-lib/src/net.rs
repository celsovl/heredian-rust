use std::convert::{TryInto};
use std::fmt;
use std::mem;
use std::time::{Duration};
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError, RecvTimeoutError};
use std::thread;

use super::*;

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

    pub fn try_recv(&self) -> Result<TMsg, TryRecvError> {
        self.chan.as_ref().unwrap().1.try_recv()
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<TMsg, RecvTimeoutError> {
        self.chan.as_ref().unwrap().1.recv_timeout(timeout)
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
                    Err(e) => panic!("{}", e)
                }

                if !done_something {
                    thread::sleep(Duration::from_millis(16));
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
    Disconnected(SocketAddr)
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
        match msg {
            Message::Accepted(_) | Message::Disconnected(_) => panic!("This type of message is not allowed to be sent."),
            _ => ()
        }

        self.chan.as_ref().unwrap().0.send(msg).unwrap();
    }

    pub fn try_recv(&self) -> Result<Message<TMsg>, TryRecvError> {
        self.chan.as_ref().unwrap().1.try_recv()
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

                let mut errors = Vec::new();

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
                                    .for_each(|(stream, addr)| {
                                        if let Err(_) = stream.write_all(&packet_bytes) {
                                            errors.push(*addr);
                                        }
                                    });
                            },
                            Message::BroadcastExcept(msg, client_addr) => {
                                let packet = Packet::<TMsg>::new(msg);
                                let packet_bytes = packet.to_bytes();

                                clients
                                    .iter_mut()
                                    .filter(|(_, addr)| *addr != client_addr)
                                    .for_each(|(stream, addr)| {
                                        if let Err(_) = stream.write_all(&packet_bytes) {
                                            errors.push(*addr);
                                        }
                                    });
                            },
                            Message::Direct(msg, client_addr) => {
                                let (stream, addr) = clients.iter_mut().find(|(_, addr)| *addr == client_addr).unwrap();
                                let packet = Packet::<TMsg>::new(msg);
                                let packet_bytes = packet.to_bytes();

                                if let Err(_) = stream.write_all(&packet_bytes) {
                                    errors.push(*addr);
                                }

                            },
                            _ => unreachable!(),
                        }

                        done_something = true;
                    },
                    Err(TryRecvError::Disconnected) => println!("Can't send message. Channel disconnected."),
                    Err(TryRecvError::Empty) => ()
                }

                // something went wrong
                if !errors.is_empty() {
                    // notify server
                    errors.iter().for_each(|addr| {
                        r_tx.send(Message::Disconnected(*addr)).unwrap();
                    });

                    // clear fault stream
                    clients.retain(|(_, addr)| !errors.contains(addr));

                    // clear errors
                    errors.clear();
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
                        Err(_) => errors.push(*client_addr),
                    }
                }

                // something went wrong
                if !errors.is_empty() {
                    // notify server
                    errors.iter().for_each(|addr| {
                        r_tx.send(Message::Disconnected(*addr)).unwrap();
                    });

                    // clear fault stream
                    clients.retain(|(_, addr)| !errors.contains(addr));

                    // clear errors
                    errors.clear();
                }

                if !done_something {
                    thread::sleep(Duration::from_millis(16));
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
