use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
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
        }
        Err(e) => {
            println!("Failed to parse input: {:?}", e);
        }
    }
}
