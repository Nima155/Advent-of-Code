macro_rules! reducer {
    ($op: tt, register, $operations: expr, $registers: expr, $ipi: expr, $ip: expr) => {{
        let mut new_registers = $registers.clone();
        new_registers[$ipi] = $ip;
        new_registers[$operations[3] as usize] =
            new_registers[$operations[2] as usize] $op new_registers[$operations[1] as usize];
        
        Self {
            instruction_pointer: new_registers[$ipi] + 1,
            registers: new_registers,
            operation: Some($operations),
            instruction_pointer_indx: $ipi,
        }
    }};

    ($op: tt, value, $operations: expr, $registers: expr, $ipi: expr, $ip: expr) => {{
        let mut new_registers = $registers.clone();
        // println!("{}", "hi");
        new_registers[$ipi] = $ip;
        new_registers[$operations[3] as usize] =
        $operations[2]  $op  new_registers[$operations[1] as usize];
        // println!("{}", new_registers[$operations[3] as usize]);
        
        Self {
            instruction_pointer: new_registers[$ipi] + 1,        
            registers: new_registers,
            operation: Some($operations),
            instruction_pointer_indx: $ipi,
        }
    }};
}

#[derive(Clone)]
pub struct Controller {
    pub registers: Vec<i64>,
    operation: Option<Vec<i64>>,
    instruction_pointer_indx: usize,
    instruction_pointer: i64,
}

impl Controller {
    pub fn new(initial_state: Vec<i64>, ipi: usize) -> Self {
        Self {
            registers: initial_state,
            operation: None,
            instruction_pointer_indx: ipi,
            instruction_pointer: 0,
        }
    }
    
    pub fn new_ops(&mut self, operations: &[i64]) {
        let mut news = vec![0];
        news.extend_from_slice(operations);
        
        self.operation = Some(news);
    }

    pub fn test_ip(&self, test_against: usize) -> Option<usize> {

        if self.instruction_pointer < test_against as i64 && self.instruction_pointer >= 0 {
            Some(self.instruction_pointer as usize)
        } else {
            None
        }
    }

    pub fn add_i(self) -> Self {
        reducer!(+, value, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn add_r(self) -> Self {
        reducer!(+, register, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn mul_i(self) -> Self {
        reducer!(*, value, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn mul_r(self) -> Self {
        reducer!(*, register, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn ban_i(self) -> Self {
        reducer!(&, value, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn ban_r(self) -> Self {
        reducer!(&, register, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn bor_i(self) -> Self {
        reducer!(|, value, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn bor_r(self) -> Self {
        reducer!(|, register, self.operation.clone().unwrap(), self.registers, self.instruction_pointer_indx, self.instruction_pointer)
    }

    pub fn set_r(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;

        new_registers[self.operation.clone().unwrap()[3] as usize] =
            new_registers[self.operation.clone().unwrap()[1] as usize];


        Self {
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,
            instruction_pointer_indx: self.instruction_pointer_indx,    
        }
    }

    pub fn set_i(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;

        new_registers[self.operation.clone().unwrap()[3] as usize] =
            self.operation.clone().unwrap()[1];

        Self {
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,
            instruction_pointer_indx: self.instruction_pointer_indx,
        }
    }

    pub fn gti_r(self) -> Self {
        let mut new_registers = self.registers.clone();

        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;

        new_registers[self.operation.clone().unwrap()[3] as usize] =
            if self.operation.clone().unwrap()[1]
                > new_registers[self.operation.clone().unwrap()[2] as usize]
            {
                1
            } else {
                0
            };


        Self {
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,
            instruction_pointer_indx: self.instruction_pointer_indx,
        }
    }

    pub fn gtr_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;
        new_registers[self.operation.clone().unwrap()[3] as usize] =
            if self.operation.clone().unwrap()[2]
                < new_registers[self.operation.clone().unwrap()[1] as usize]
            {
                1
            } else {
                0
            };

        Self {
            instruction_pointer_indx: self.instruction_pointer_indx,
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,
        }
    }

    pub fn gtr_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;
        new_registers[self.operation.clone().unwrap()[3] as usize] = if new_registers
            [self.operation.clone().unwrap()[1] as usize]
            > new_registers[self.operation.clone().unwrap()[2] as usize]
        {
            1
        } else {
            0
        };
        
        Self {
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,

            instruction_pointer_indx: self.instruction_pointer_indx,
        }
    }

    pub fn eqi_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;
        new_registers[self.operation.clone().unwrap()[3] as usize] =
            if self.operation.clone().unwrap()[1]
                == new_registers[self.operation.clone().unwrap()[2] as usize]
            {
                1
            } else {
                0
            };
        Self {
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,

            instruction_pointer_indx: self.instruction_pointer_indx,
        }
    }

    pub fn eqr_i(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;
        new_registers[self.operation.clone().unwrap()[3] as usize] = if new_registers
            [self.operation.clone().unwrap()[1] as usize]
            == self.operation.clone().unwrap()[2]
        {
            1
        } else {
            0
        };
        Self {
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,
            instruction_pointer_indx: self.instruction_pointer_indx,
        }
    }

    pub fn eqr_r(self) -> Self {
        let mut new_registers = self.registers.clone();
        new_registers[self.instruction_pointer_indx] = self.instruction_pointer as i64;

        new_registers[self.operation.clone().unwrap()[3] as usize] = if new_registers
            [self.operation.clone().unwrap()[1] as usize]
            == new_registers[self.operation.clone().unwrap()[2] as usize]
        {
            1
        } else {
            0
        };
        
        Self {
            instruction_pointer: new_registers[self.instruction_pointer_indx] + 1,
            registers: new_registers,
            operation: self.operation,
            instruction_pointer_indx: self.instruction_pointer_indx,
        }
    }
}
