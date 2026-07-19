use std::ops::{Add, Sub};
use yarnspinner_core::prelude::Destination;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct InstructionAddress(usize);

impl InstructionAddress {
    pub(crate) const ZERO: InstructionAddress = InstructionAddress(0);

    pub fn default() -> Self {
        Self::ZERO
    }

    pub fn new(address: usize) -> Self {
        Self(address)
    }

    pub fn value(&self) -> usize { self.0 }
}

impl From<InstructionAddress> for usize {
    fn from(value: InstructionAddress) -> Self {
        value.0
    }
}

impl TryFrom<InstructionAddress> for i32 {
    type Error = <i32 as TryFrom<usize>>::Error;

    fn try_from(value: InstructionAddress) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl Add<isize> for InstructionAddress {
    type Output = InstructionAddress;

    fn add(self, rhs: isize) -> Self::Output {
        if rhs > 0 {
            InstructionAddress(self.0 + rhs as usize)
        } else {
            InstructionAddress(self.0 - rhs as usize)
        }
    }
}

impl Sub<isize> for InstructionAddress {
    type Output = InstructionAddress;

    fn sub(self, rhs: isize) -> Self::Output {
        if rhs > 0 {
            InstructionAddress(self.0 - rhs as usize)
        } else {
            InstructionAddress(self.0 + rhs as usize)
        }
    }
}

impl Add<usize> for InstructionAddress {
    type Output = InstructionAddress;

    fn add(self, rhs: usize) -> Self::Output {
        InstructionAddress(self.0 + rhs)
    }
}

impl Sub<usize> for InstructionAddress {
    type Output = InstructionAddress;

    fn sub(self, rhs: usize) -> Self::Output {
        InstructionAddress(self.0 - rhs)
    }
}

impl Sub<InstructionAddress> for InstructionAddress {
    type Output = isize;

    fn sub(self, rhs: InstructionAddress) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        if lhs > rhs {
            (lhs - rhs) as isize
        } else {
            - ( (rhs - lhs) as isize )
        }
    }
}

pub(crate) fn range(from: InstructionAddress, to: InstructionAddress) -> impl Iterator<Item = InstructionAddress> {
    let from = from.0;
    let to = to.0;
    
    (from..to).map(|a| InstructionAddress::new(a))
}

trait DestinationAddress: Destination {
    fn destination_addr(&self) -> InstructionAddress
    {
        InstructionAddress::new(self.destination())
    }

    fn set_destination_addr(&mut self, destination: InstructionAddress)
    {
        self.set_destination(destination.value());
    }
}

impl<T: Destination> DestinationAddress for T {}
