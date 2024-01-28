#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_variables)]
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    cell::RefCell,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
struct Packet {
    id: u64,
    vers: Rc<RefCell<u64>>,
    subpackets: Vec<Rc<RefCell<Packet>>>,
    value: Rc<RefCell<u64>>,
}

impl Hash for Packet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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
    fn add_child(&mut self, child: Rc<RefCell<Packet>>) {
        self.subpackets.push(child);
    }
    fn sum_vers(&self) -> u64 {
        let mut sum = 0;
        for c in self.subpackets.iter() {
            sum += c.borrow().sum_vers()
        }
        *self.vers.borrow() + sum
    }
    fn find_values(&mut self) {
        let mut values = Vec::new();
        for child in self.subpackets.iter() {
            child.borrow_mut().find_values();
            values.push(*child.borrow().value.borrow());
        }
        if !values.is_empty() {
            *self.value.borrow_mut() = match self.id {
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => if values[0] > values[1] {1} else {0},
                6 => if values[0] < values[1] {1} else {0},
                7 => if values[0] == values[1] {1} else {0},
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

fn to_dcm(slice: &str) -> u64 {
    u64::from_str_radix(slice, 2).expect("error converting to decimal")
}

fn to_bin(hex: &str) -> String {
    hex.chars().map(|c| {
        u8::from_str_radix(&c.to_string(), 16)
        .map(|num| format!("{:04b}", num))
        .expect("error converting to binary")
    }).collect() 
}

fn parse_value(slice: &str, id: usize) -> (u64, usize) {
    let mut idx = id;
    let mut value = String::new();

    loop {
        let next_nibble = &slice[idx..idx + 5];
        value.push_str(&next_nibble[1..]);
        idx += 5;
        if next_nibble.starts_with('0') { break }
    }

    (to_dcm(&value), idx)
}

fn parse_pkt(slice: &str, s_idx: usize) -> Option<(Rc<RefCell<Packet>>, usize)> {
    if s_idx >= slice.len() { return None; }

    let pkt = Rc::new(RefCell::new(Packet::new()));
    let mut idx = s_idx;

    let pkt_ver = to_dcm(&slice[idx..idx + 3]);
    let pkt_id = to_dcm(&slice[idx + 3..idx + 6]);
    idx += 6;

    pkt.borrow_mut().vers.replace(pkt_ver);
    pkt.borrow_mut().id = pkt_id;

    if pkt_id == 4 {
        let (val, n_idx) = parse_value(&slice, idx);
        pkt.borrow_mut().value.replace(val);
        idx = n_idx;
    } else {
        let len_type = &slice[idx..idx + 1];
        idx += 1;
        if len_type.starts_with("0") {
            let sub_len = to_dcm(&slice[idx..idx + 15]) as usize;
            idx += 15;
            let end_idx = idx + sub_len;
            while idx < end_idx {
                if let Some((spkt, n_idx)) = parse_pkt(&slice, idx) {
                    idx = n_idx;
                    pkt.borrow_mut().subpackets.push(spkt);
                } else { break }
            }
        } else {
            let spkt_count = to_dcm(&slice[idx..idx + 11]) as usize;
            idx += 11;
            for _ in 0..spkt_count {
                if let Some((spkt, n_idx)) = parse_pkt(&slice, idx) {
                    idx = n_idx;
                    pkt.borrow_mut().subpackets.push(spkt);
                } else { break }
            }
        }
    }
    Some((pkt, idx))
}

fn part1(input: &str) -> u64 {
    let binary_input = to_bin(input);
    let (packet, _) = parse_pkt(&binary_input, 0).unwrap();
    //packet.borrow().print_tree(1);
    let sum = packet.borrow().sum_vers();
    sum
}

fn part2(input: &str) -> u64 {
    let binary_input = to_bin(input);
    let (packet, _) = parse_pkt(&binary_input, 0).unwrap();
    //packet.borrow().print_tree(1);
    packet.borrow_mut().find_values();
    let sum = *packet.borrow().value.borrow();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
    }
    #[test]
    fn test_2() {
        assert_eq!(54, part2("04005AC33890"));
    }
}
