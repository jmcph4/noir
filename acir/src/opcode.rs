#[derive(Clone, Debug, Hash, Copy)]
pub enum OPCODE {
    AES,
    SHA256
}

impl std::fmt::Display for OPCODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl OPCODE{
 pub fn to_u16(&self) -> u16 {
     match self {
        OPCODE::AES => 0,
        OPCODE::SHA256 => 1
     }
 }   
 pub fn name(&self) -> &str {
     match self {
         OPCODE::AES => "aes",
         OPCODE::SHA256 => "sha256",
     }
 }
 pub fn lookup(op_name : &str) -> Option<OPCODE> {
     match op_name {
         "sha256" => Some(OPCODE::SHA256), 
         _=> None,
     }
 }
 pub fn is_valid_opcode_name(op_name : &str) -> bool {
     OPCODE::lookup(op_name).is_some()
 }
 pub fn definition(&self) -> GadgetDefinition {
     match self {
         OPCODE::AES => unimplemented!(),
         OPCODE::SHA256 => GadgetDefinition {
            name : self.name().into(),
            input_size : InputSize::Variable,
            output_size: OutputSize(2),
         },
     }
 }
}

// Descriptor as to whether the input/output is fixed or variable
// Example: The input for Sha256 is Variable and the output is fixed at 2 witnesses, each holding 128 bits of the actual sha256 function
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum InputSize {
    Variable,
    Fixed(u128),
}

// Output size Cannot currently vary, so we use a separate struct
// XXX: In the future, we may be able to allow the output to vary based on the input size, however this implies support for dynamic supports
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct OutputSize(pub u128);

#[derive(Clone, Debug, Hash)]
// Specs for how many inputs/outputs the method takes.
pub struct GadgetDefinition {
    pub name: String,
    pub input_size: InputSize,
    pub output_size: OutputSize,
}