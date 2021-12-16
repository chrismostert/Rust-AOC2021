use bitvec::prelude::*;

struct Reader {
    bits: BitVec<Msb0, u8>,
    pos: usize,
}
impl Reader {
    fn slice(&mut self, size: usize) -> &BitSlice<Msb0, u8> {
        let res = &self.bits[self.pos..self.pos + size];
        self.pos += size;
        res
    }
    fn bit(&mut self) -> bool {
        let res = self.bits[self.pos];
        self.pos += 1;
        res
    }
    fn load<T: bitvec::mem::BitMemory>(&mut self, size: usize) -> T {
        self.slice(size).load_be()
    }
}

fn parse_packet(reader: &mut Reader) -> (usize, usize) {
    // Header
    let version: usize = reader.load(3);
    let type_id: usize = reader.load(3);

    // Literal value?
    if type_id == 4 {
        let mut val = BitVec::<Msb0, u8>::new();
        loop {
            let last_group = !reader.bit();
            val.extend(reader.slice(4));
            if last_group {
                return (version, val.load_be());
            }
        }
    }

    // Recursive operator call
    let mut subpackets = Vec::new();
    if reader.bit() {
        let n_packets: usize = reader.load(11);
        for _ in 0..n_packets {
            subpackets.push(parse_packet(reader));
        }
    } else {
        let n_bits: usize = reader.load(15);
        let target = reader.pos + n_bits;
        while reader.pos != target {
            subpackets.push(parse_packet(reader));
        }
    };

    // Perform operation
    let mut subvalues = subpackets.iter().map(|(_, val)| *val);
    let value = match type_id {
        0 => subvalues.sum(),
        1 => subvalues.product(),
        2 => subvalues.min().unwrap(),
        3 => subvalues.max().unwrap(),
        5 => (subvalues.next().unwrap() > subvalues.next().unwrap()) as usize,
        6 => (subvalues.next().unwrap() < subvalues.next().unwrap()) as usize,
        7 => (subvalues.next().unwrap() == subvalues.next().unwrap()) as usize,
        _ => unreachable!(),
    };

    let v_sum: usize = subpackets.iter().map(|(ver, _)| ver).sum();
    (version + v_sum, value)
}

fn main() {
    let input = include_str!("../input.txt").lines().next().unwrap();
    let bytes = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..=i + 1], 16).unwrap())
        .collect();

    let (version_sum, value) = parse_packet(&mut Reader {
        bits: BitVec::from_vec(bytes),
        pos: 0,
    });

    println!("Part 1: {}", version_sum);
    println!("Part 2: {}", value);
}
