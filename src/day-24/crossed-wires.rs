use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl FromStr for GateKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "and" => Ok(GateKind::And),
            "or" => Ok(GateKind::Or),
            "xor" => Ok(GateKind::Xor),
            _ => Err("Invalid gate type"),
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    kind: GateKind,
    lhs: String,
    rhs: String,
}

#[derive(Debug, Clone)]
struct Circuit {
    gates: HashMap<String, Gate>,
    wires: HashMap<String, u8>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            gates: HashMap::new(),
            wires: HashMap::new(),
        }
    }

    fn add_gate(&mut self, kind: GateKind, lhs: &str, rhs: &str, output: &str) {
        let gate = Gate {
            kind,
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
        };
        self.gates.insert(output.to_string(), gate);
    }

    fn add_wire_value(&mut self, wire: &str, value: u8) {
        self.wires.insert(wire.to_string(), value);
    }

    fn evaluate(&mut self, wire: &str) -> u8 {
        if let Some(&value) = self.wires.get(wire) {
            return value;
        }

        let gate = self
            .gates
            .get(wire)
            .expect(&format!("No gate for wire {}", wire))
            .clone();

        let value1 = self.evaluate(&gate.lhs);
        let value2 = self.evaluate(&gate.rhs);
        let result = match gate.kind {
            GateKind::And => value1 & value2,
            GateKind::Or => value1 | value2,
            GateKind::Xor => value1 ^ value2,
        };

        self.add_wire_value(wire, result);
        result
    }

    fn get_output_binary(&mut self) -> String {
        self.gates
            .keys()
            .filter(|gate| gate.starts_with('z'))
            .map(String::clone)
            .sorted_by(|lhs, rhs| rhs.cmp(lhs))
            .map(|gate| self.evaluate(&gate).to_string())
            .collect::<String>()
    }

    pub fn find_swapped_wires(&self) -> Vec<String> {
        let mut lookup = HashSet::new();
        for (_, gate) in self.gates.iter() {
            lookup.insert((&gate.lhs, gate.kind));
            lookup.insert((&gate.rhs, gate.kind));
        }

        let mut swapped = HashSet::new();
        for (to, gate) in self.gates.iter() {
            match gate.kind {
                GateKind::And => {
                    // check that all AND gates point to an OR gate, except for the first AND gate
                    if gate.lhs != "x00"
                        && gate.rhs != "x00"
                        && !lookup.contains(&(to, GateKind::Or))
                    {
                        swapped.insert(to.clone());
                    }
                }
                GateKind::Or => {
                    // check that only XOR gates point to output, except for the last carry (z45)
                    if to.starts_with('z') && to != "z45" {
                        swapped.insert(to.clone());
                    }
                }
                GateKind::Xor => {
                    if gate.lhs.starts_with('x') || gate.rhs.starts_with('x') {
                        // first level XOR must point to a second level XOR, except for the first XOR gate
                        if gate.lhs != "x00"
                            && gate.rhs != "x00"
                            && !lookup.contains(&(to, GateKind::Xor))
                        {
                            swapped.insert(to.clone());
                        }
                    } else {
                        // second level XOR must point to an output (z-prefixed wires)
                        if !to.starts_with('z') {
                            swapped.insert(to.clone());
                        }
                    }
                }
            }
        }

        swapped.into_iter().sorted().collect()
    }
}

#[derive(Debug)]
enum WireSystemParseError {
    InvalidGateType,
    InvalidValue,
}

impl FromStr for Circuit {
    type Err = WireSystemParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut system = Circuit::new();
        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            if let Some((wire, value)) = line.split_once(':') {
                let wire = wire.trim();
                let value = match value.trim().parse::<u8>() {
                    Ok(val) => val,
                    Err(_) => return Err(WireSystemParseError::InvalidValue),
                };
                system.add_wire_value(wire, value);
            }
        }

        for line in lines {
            let (inputs, output) = line
                .split_once("->")
                .map(|(inputs, output)| (inputs.trim(), output.trim()))
                .ok_or(WireSystemParseError::InvalidGateType)?;

            let inputs = inputs.split_whitespace().collect::<Vec<&str>>();

            let gate_type = match inputs[1].parse::<GateKind>() {
                Ok(gate) => gate,
                Err(_) => return Err(WireSystemParseError::InvalidGateType),
            };

            let input1 = inputs[0];
            let input2 = inputs[2];

            system.add_gate(gate_type, input1, input2, output);
        }

        Ok(system)
    }
}

fn main() {
    let input = include_str!("input.data");
    match input.parse::<Circuit>() {
        Ok(mut system) => {
            let timer = std::time::Instant::now();
            let binary_output = system.get_output_binary();
            let decimal_output = i64::from_str_radix(&binary_output, 2).unwrap();
            println!("Binary output: {}", binary_output);
            println!("Decimal output: {}", decimal_output);
            println!("The time spent is {:?}", timer.elapsed());

            let timer = std::time::Instant::now();
            println!(
                "Names of eight names involved in swap: {}",
                system.find_swapped_wires().join(",")
            );
            println!("The time spent is {:?}", timer.elapsed());
        }
        Err(e) => {
            println!("Failed to parse input: {:?}", e);
        }
    }
}
