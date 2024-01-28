use std::{
    rc::Rc,
    cell::RefCell,
};

#[derive(Debug, Clone)]
struct Packet {
    id: u64,
    vers: Rc<RefCell<u64>>,
    subpackets: Vec<Rc<RefCell<Packet>>>,
    value: Rc<RefCell<u64>>,
}

impl Packet {
    fn new() -> Packet {
        Packet {
            id: 0,
            vers: Rc::new(RefCell::new(0)),
            subpackets: Vec::new(),
            value: Rc::new(RefCell::new(0)),
        }  
    }
    fn sum_vers(&self) -> u64 {
        let mut sum = 0;
        for c in self.subpackets.iter() {
            sum += c.borrow().sum_vers()
        }
        *self.vers.borrow() + sum
    }
    fn find_values(&mut self) {
        let mut v = Vec::new();
        for child in self.subpackets.iter() {
            child.borrow_mut().find_values();
            v.push(*child.borrow().value.borrow());
        }
        if !v.is_empty() {
            *self.value.borrow_mut() = match self.id {
                0 => v.iter().sum(),
                1 => v.iter().product(),
                2 => *v.iter().min().unwrap(),
                3 => *v.iter().max().unwrap(),
                5 => if v[0] > v[1] {1} else {0},
                6 => if v[0] < v[1] {1} else {0},
                7 => if v[0] == v[1] {1} else {0},
                _ => *self.value.borrow(),
            }
        }
    }
    fn print_tree(&self, indent: usize) {
        let indent_str = " ".repeat(indent * 2);
        println!("{} id: {}, version: {}, value: {}", indent_str, self.id, self.vers.borrow(), self.value.borrow());
        for child in &self.subpackets {
            child.borrow().print_tree(indent + 1);
        }
    }
}

fn to_dcm(s: &str) -> u64 {
    u64::from_str_radix(s, 2).expect("error converting to decimal")
}

fn to_bin(hex: &str) -> String {
    hex.chars().map(|c| {
        u8::from_str_radix(&c.to_string(), 16)
        .map(|num| format!("{:04b}", num))
        .expect("error converting to binary")
    }).collect() 
}

fn parse_value(s: &str, id: usize) -> (u64, usize) {
    let mut idx = id;
    let mut value = String::new();

    loop {
        let next_nibble = &s[idx..idx + 5];
        value.push_str(&next_nibble[1..]);
        idx += 5;
        if next_nibble.starts_with('0') { break }
    }

    (to_dcm(&value), idx)
}

fn parse_pkt(s: &str, s_idx: usize) -> Option<(Rc<RefCell<Packet>>, usize)> {
    if s_idx >= s.len() { return None; }

    let pkt = Rc::new(RefCell::new(Packet::new()));
    let mut idx = s_idx;

    let pkt_ver = to_dcm(&s[idx..idx + 3]);
    let pkt_id = to_dcm(&s[idx + 3..idx + 6]);
    idx += 6;

    pkt.borrow_mut().vers.replace(pkt_ver);
    pkt.borrow_mut().id = pkt_id;

    if pkt_id == 4 {
        let (val, n_idx) = parse_value(&s, idx);
        pkt.borrow_mut().value.replace(val);
        idx = n_idx;
    } else {
        let len_type = &s[idx..idx + 1];
        idx += 1;
        if len_type.starts_with("0") {
            let sub_len = to_dcm(&s[idx..idx + 15]) as usize;
            idx += 15;
            let end_idx = idx + sub_len;
            while idx < end_idx {
                if let Some((spkt, n_idx)) = parse_pkt(&s, idx) {
                    idx = n_idx;
                    pkt.borrow_mut().subpackets.push(spkt);
                } else { break }
            }
        } else {
            let spkt_count = to_dcm(&s[idx..idx + 11]) as usize;
            idx += 11;
            for _ in 0..spkt_count {
                if let Some((spkt, n_idx)) = parse_pkt(&s, idx) {
                    idx = n_idx;
                    pkt.borrow_mut().subpackets.push(spkt);
                } else { break }
            }
        }
    }
    Some((pkt, idx))
}

fn solve(input: &str) -> (u64, u64) {
    let s = to_bin(input);
    let (packet, _) = parse_pkt(&s, 0).unwrap();
    let vrs_sum = packet.borrow().sum_vers();
    //packet.borrow().print_tree(1);
    packet.borrow_mut().find_values();
    let value = *packet.borrow().value.borrow();
    (vrs_sum, value)
}
fn main() {
    let input = include_str!("input16.txt");
    let (part1, part2) = solve(input);
    println!("{}, {}", part1, part2);
}