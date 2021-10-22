#[derive(Clone)]
pub struct Controller {
    pub registers: Vec<i64>,
    operation: Vec<i64>,
}

impl Controller {
    pub fn new(initial_state: Vec<i64>, operation: Vec<i64>) -> Self {
        Self {
            registers: initial_state,
            operation,
        }
    }

    pub fn add_i(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.operation[2] + self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn add_r(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.registers[self.operation[2] as usize] + self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn mul_i(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.operation[2] * self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn mul_r(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.registers[self.operation[2] as usize] * self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn ban_i(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.operation[2] & self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn ban_r(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.registers[self.operation[2] as usize] & self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn bor_i(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.operation[2] | self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn bor_r(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.operation[3] as usize] =
            self.registers[self.operation[2] as usize] | self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn set_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] = self.registers[self.operation[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn set_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] = self.operation[1];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn gti_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] =
            if self.operation[1] > self.registers[self.operation[2] as usize] {
                1
            } else {
                0
            };

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn gtr_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] =
            if self.operation[2] < self.registers[self.operation[1] as usize] {
                1
            } else {
                0
            };

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn gtr_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] = if self.registers[self.operation[1] as usize]
            > self.registers[self.operation[2] as usize]
        {
            1
        } else {
            0
        };

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn eqi_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] =
            if self.operation[1] == self.registers[self.operation[2] as usize] {
                1
            } else {
                0
            };

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn eqr_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] =
            if self.registers[self.operation[1] as usize] == self.operation[2] {
                1
            } else {
                0
            };

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn eqr_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation[3] as usize] = if self.registers[self.operation[1] as usize]
            == self.registers[self.operation[2] as usize]
        {
            1
        } else {
            0
        };

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }
}

impl PartialEq for Controller {
    fn eq(&self, other: &Self) -> bool {
        self.registers == other.registers
    }
}