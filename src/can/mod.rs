pub struct BaseID(u16);
pub struct ExtendedID(u32);

/// A can ID, can either be Extended (29bit CAN2.0B) or Base (normal 11bit CAN2.0A)
pub enum ID{
    BaseID(BaseID),
    ExtendedID(ExtendedID),
}

pub struct DataFrame {
    id: ID,
    dlc: u8,
    data: [u8; 8],
}

pub struct RemoteFrame {
    id: ID,
    dlc: u8,
}

impl DataFrame {
    pub fn new(id: ID) -> DataFrame {
        DataFrame{id: id, dlc: 0, data: [0; 8]}
    }
    
    pub fn set_data_length(&mut self, length: usize) {
        assert!(length <= 8);
        self.dlc = length as u8;
    }
    
    pub fn data(&self) -> &[u8] {
        &self.data[0..(self.dlc as usize)]
    }
    
    pub fn data_as_mut(&mut self) -> &mut[u8] {
        &mut self.data[0..(self.dlc as usize)]
    }
    
    pub fn id(&self) -> &ID {
        &self.id 
    }
}

impl RemoteFrame {
    pub fn new(id: ID) -> RemoteFrame {
        RemoteFrame{id: id, dlc: 0}
    }
    
    pub fn set_data_length(&mut self, length: usize) {
        assert!(length <= 8);
        self.dlc = length as u8;
    }
        
    pub fn id(&self) -> &ID {
        &self.id 
    }
}
