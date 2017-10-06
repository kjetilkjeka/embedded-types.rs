
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BaseID(u16);

impl BaseID {
    pub fn new(id: u16) -> Self {
        assert_eq!(id & 0xf800, 0);
        BaseID(id)
    }
}

impl From<BaseID> for u16 {
    fn from(id: BaseID) -> Self {
        id.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtendedID(u32);

impl ExtendedID {
    pub fn new(id: u32) -> Self {
        assert_eq!(id & 0xe000_0000, 0);
        ExtendedID(id)
    }
}

impl From<ExtendedID> for u32 {
    fn from(id: ExtendedID) -> Self {
        id.0
    }
}

/// A can ID, can either be Extended (29bit CAN2.0B) or Base (normal 11bit CAN2.0A)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ID{
    /// ID for CAN2.0A data frames
    BaseID(BaseID),
    
    /// ID for CAN2.0B data frames
    ExtendedID(ExtendedID),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BaseDataFrame {
    id: BaseID,
    dlc: u8,
    data: [u8; 8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtendedDataFrame {
    id: ExtendedID,
    dlc: u8,
    data: [u8; 8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataFrame {
    /// A CAN2.0A data frame
    BaseDataFrame(BaseDataFrame),
    
    /// A CAN2.0B data frame
    ExtendedDataFrame(ExtendedDataFrame),
}    

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RemoteFrame {
    id: ID,
    dlc: u8,
}

impl ExtendedDataFrame {
    pub fn new(id: ExtendedID) -> Self {
        Self{id: id, dlc: 0, data: [0; 8]}
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
    
    pub fn id(&self) -> ExtendedID {
        self.id 
    }
}

impl BaseDataFrame {
    pub fn new(id: BaseID) -> Self {
        Self{id: id, dlc: 0, data: [0; 8]}
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
    
    pub fn id(&self) -> BaseID {
        self.id 
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
        
    pub fn id(&self) -> ID {
        self.id 
    }
}
