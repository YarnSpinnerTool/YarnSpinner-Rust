use std::mem;
use yarnspinner_core::prelude::{Destination, Header, Instruction, Node};
use crate::output::instruction_address::InstructionAddress;
use crate::output::range;

pub struct NodeBuilder {
    pub name: String,
    pub headers: Vec<Header>,
    instructions: Vec<InstructionCell>,
}

#[derive(Clone)]
pub struct InstructionCell {
    instruction: Instruction,
    address: InstructionAddress
}

impl InstructionCell {
    pub fn address(&self) -> InstructionAddress {
        self.address
    }

    pub fn get(&self) -> &Instruction { &self.instruction }

    pub fn get_mut(&mut self) -> &mut Instruction { &mut self.instruction }

    pub fn set(&mut self, instruction: impl Into<Instruction>)
    {
        self.instruction = instruction.into();
    }
}

impl NodeBuilder {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned(), headers: Vec::new(), instructions: Vec::new() }
    }

    fn ensure_reserve(&mut self, increase: usize) {
        self.instructions.reserve(increase);
    }

    pub fn last_address(&self) -> Option<InstructionAddress> {
        Some(self.instructions.last()?.address)
    }

    /// Returns the current instruction pointer.
    /// This points just past the last emitted instruction.
    ///
    /// When no instructions have been emitted the instruction pointer points to 0.
    pub fn next_address(&self) -> InstructionAddress {
        if let Some(InstructionCell { address, .. }) = self.instructions.last().as_mut() {
            address.clone() + 1usize
        } else {
            InstructionAddress::ZERO
        }
    }

    pub fn append_instruction(&mut self, instruction: impl Into<Instruction>) -> InstructionAddress {
        self.append_instruction_mut(instruction).address
    }

    pub fn append_instruction_mut(&mut self, instruction: impl Into<Instruction>) -> &mut InstructionCell {
        self.ensure_reserve(1);

        self.instructions.push_mut(InstructionCell {
            instruction: instruction.into(),
            address: self.next_address()
        })
    }

    pub fn set_instruction(&mut self, address: InstructionAddress, instruction: impl Into<Instruction>) -> InstructionAddress {
        let diff = address + 1usize - self.next_address();
        if diff > 0 {
            self.reserve_instructions(diff as usize);
        }

        self.get_instruction_mut(address).unwrap().set(instruction);
        address
    }

    pub fn reserve_instructions(&mut self, count: usize) -> Vec<InstructionAddress> {
        if count == 0 { return Vec::new() }

        self.ensure_reserve(count);

        let mut result = Vec::new();

        let start = self.next_address();
        for address in range(start, start+count) {
            self.instructions.push(InstructionCell {
                instruction: Instruction::default(),
                address
            });

            result.push(address);
        }

        result
    }

    pub fn reserve_instructions_mut(&mut self, count: usize) -> &mut [InstructionCell] {
        if count == 0 { return &mut [] }
        self.ensure_reserve(count);

        let addr = self.next_address();
        self.reserve_instructions(count);
        &mut self.instructions[addr.value()..]
    }

    pub fn get_instruction(&self, address: InstructionAddress) -> Option<&InstructionCell> {
        self.instructions.get(address.value())
    }
    pub fn get_instruction_mut(&mut self, address: InstructionAddress) -> Option<&mut InstructionCell> {
        self.instructions.get_mut(address.value())
    }

    pub fn set(&mut self, mut cell: InstructionCell) -> &mut InstructionCell {
        let instruction = mem::replace(&mut cell.instruction, Instruction::default());
        let addr = self.set_instruction(cell.address, instruction);
        mem::forget(cell);
        self.get_instruction_mut(addr).unwrap()
    }

    pub fn take(node: Node) -> Self {
        let mut builder = Self {
            name: node.name,
            headers: node.headers,
            instructions: Vec::new()
        };

        for inst in node.instructions {
            builder.append_instruction(inst);
        }

        builder
    }

    pub fn set_destination_at(&mut self, at: InstructionAddress, to: InstructionAddress) -> Result<(), ()> {
        let Some(inst) = self.get_instruction_mut(at) else { return Err(()); };
        let Some(inst): Option<&mut dyn Destination> = inst.get_mut().try_into().ok() else { return Err(()); };

        inst.set_destination(to.value());

        Ok(())
    }

    pub fn build(self) -> Node {
        let mut node = Node::default();
        self.build_in(&mut node);
        node
    }

    pub fn build_in(mut self, node: &mut Node) {
        node.name = mem::replace(&mut self.name, "".to_owned());
        node.headers = mem::replace(&mut self.headers, Vec::new());
        let instructions = mem::replace(&mut self.instructions, Vec::new());

        for cell in instructions {
            node.instructions.push(cell.instruction.clone());
            mem::forget(cell);
        }
    }
}

impl Drop for InstructionCell {
    fn drop(&mut self) {
        panic!("must not drop InstructionCell");
    }
}

impl Drop for NodeBuilder {
    fn drop(&mut self) {
        let instructions = mem::replace(&mut self.instructions, Vec::new());
        for cell in instructions {
            mem::forget(cell);
        }
    }
}