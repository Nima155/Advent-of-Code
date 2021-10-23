macro_rules! reducer {
    ($op: tt, register, $operations: expr, $registers: expr) => {{
        let mut new_registers = $registers.clone();

        new_registers[$operations[3] as usize] =
            $registers[$operations[2] as usize] $op $registers[$operations[1] as usize];

        Self {
            registers: new_registers,
            operation: Some($operations),
        }
    }};

    ($op: tt, value, $operations: expr, $registers: expr) => {{
        let mut new_registers = $registers.clone();

        new_registers[$operations[3] as usize] =
            $operations[2]  $op  $registers[$operations[1] as usize];

        Self {
            registers: new_registers,
            operation: Some($operations),
        }
    }};
}

#[derive(Clone)]
pub struct Controller {
    pub registers: Vec<i64>,
    operation: Option<Vec<i64>>,
}

impl Controller {
    pub fn new(initial_state: Vec<i64>) -> Self {
        Self {
            registers: initial_state,
            operation: None,
        }
    }
    pub fn new_ops(&mut self, operations: &[i64]) {
        self.operation = Some(operations.to_owned());
    }

    pub fn add_i(self) -> Self {
        reducer!(+, value, self.operation.clone().unwrap(), self.registers)
    }

    pub fn add_r(self) -> Self {
        reducer!(+, register, self.operation.clone().unwrap(), self.registers)
    }

    pub fn mul_i(self) -> Self {
        reducer!(*, value, self.operation.clone().unwrap(), self.registers)
    }

    pub fn mul_r(self) -> Self {
        reducer!(*, register, self.operation.clone().unwrap(), self.registers)
    }

    pub fn ban_i(self) -> Self {
        reducer!(&, value, self.operation.clone().unwrap(), self.registers)
    }

    pub fn ban_r(self) -> Self {
        reducer!(&, register, self.operation.clone().unwrap(), self.registers)
    }

    pub fn bor_i(self) -> Self {
        reducer!(|, value, self.operation.clone().unwrap(), self.registers)
    }

    pub fn bor_r(self) -> Self {
        reducer!(|, register, self.operation.clone().unwrap(), self.registers)
    }

    pub fn set_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation.clone().unwrap()[3] as usize] =
            self.registers[self.operation.clone().unwrap()[1] as usize];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn set_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation.clone().unwrap()[3] as usize] =
            self.operation.clone().unwrap()[1];

        Self {
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn gti_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation.clone().unwrap()[3] as usize] =
            if self.operation.clone().unwrap()[1]
                > self.registers[self.operation.clone().unwrap()[2] as usize]
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

    pub fn gtr_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation.clone().unwrap()[3] as usize] =
            if self.operation.clone().unwrap()[2]
                < self.registers[self.operation.clone().unwrap()[1] as usize]
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

    pub fn gtr_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation.clone().unwrap()[3] as usize] = if self.registers
            [self.operation.clone().unwrap()[1] as usize]
            > self.registers[self.operation.clone().unwrap()[2] as usize]
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
        new_registers[self.operation.clone().unwrap()[3] as usize] =
            if self.operation.clone().unwrap()[1]
                == self.registers[self.operation.clone().unwrap()[2] as usize]
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

    pub fn eqr_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation.clone().unwrap()[3] as usize] = if self.registers
            [self.operation.clone().unwrap()[1] as usize]
            == self.operation.clone().unwrap()[2]
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

    pub fn eqr_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.operation.clone().unwrap()[3] as usize] = if self.registers
            [self.operation.clone().unwrap()[1] as usize]
            == self.registers[self.operation.clone().unwrap()[2] as usize]
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
