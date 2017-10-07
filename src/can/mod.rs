
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

impl From<ID> for u32 {
    fn from(id: ID) -> Self {
        match id {
            ID::BaseID(x) => u16::from(x) as u32,
            ID::ExtendedID(x) => u32::from(x),
        }
    }    
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
pub struct BaseRemoteFrame {
    id: BaseID,
    dlc: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtendedRemoteFrame {
    id: ExtendedID,
    dlc: u8,
}

pub enum RemoteFrame {
    /// A CAN2.0A remote frame
    BaseRemoteFrame(BaseRemoteFrame),
    
    /// A CAN2.0B remote frame
    ExtendedRemoteFrame(ExtendedRemoteFrame),
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

impl DataFrame {
    pub fn new(id: ID) -> Self {
        match id {
            ID::BaseID(id) => DataFrame::BaseDataFrame(BaseDataFrame::new(id)),
            ID::ExtendedID(id) => DataFrame::ExtendedDataFrame(ExtendedDataFrame::new(id)),
        }
    }
    
    pub fn set_data_length(&mut self, length: usize) {
        match *self {
            DataFrame::BaseDataFrame(ref mut f) => f.set_data_length(length),
            DataFrame::ExtendedDataFrame(ref mut f) => f.set_data_length(length),
        }
    }
    
    pub fn data(&self) -> &[u8] {
        match *self {
            DataFrame::BaseDataFrame(ref f) => f.data(),
            DataFrame::ExtendedDataFrame(ref f) => f.data(),
        }
    }
    
    pub fn data_as_mut(&mut self) -> &mut[u8] {
        match *self {
            DataFrame::BaseDataFrame(ref mut f) => f.data_as_mut(),
            DataFrame::ExtendedDataFrame(ref mut f) => f.data_as_mut(),
        }
    }
     
    pub fn id(&self) -> ID {
        match *self {
            DataFrame::BaseDataFrame(f) => ID::BaseID(f.id()),
            DataFrame::ExtendedDataFrame(f) => ID::ExtendedID(f.id()),
        }
    }
}

impl BaseRemoteFrame {
    pub fn new(id: BaseID) -> Self {
        Self{id: id, dlc: 0}
    }
    
    pub fn set_data_length(&mut self, length: usize) {
        assert!(length <= 8);
        self.dlc = length as u8;
    }
        
    pub fn id(&self) -> BaseID {
        self.id 
    }
}

impl ExtendedRemoteFrame {
    pub fn new(id: ExtendedID) -> Self {
        Self{id: id, dlc: 0}
    }
    
    pub fn set_data_length(&mut self, length: usize) {
        assert!(length <= 8);
        self.dlc = length as u8;
    }
        
    pub fn id(&self) -> ExtendedID {
        self.id 
    }
}

impl RemoteFrame {
    pub fn new(id: ID) -> Self {
        match id {
            ID::BaseID(id) => RemoteFrame::BaseRemoteFrame(BaseRemoteFrame::new(id)),
            ID::ExtendedID(id) => RemoteFrame::ExtendedRemoteFrame(ExtendedRemoteFrame::new(id)),
        }
    }
    
    pub fn id(&self) -> ID {
        match *self {
            RemoteFrame::BaseRemoteFrame(f) => ID::BaseID(f.id()),
            RemoteFrame::ExtendedRemoteFrame(f) => ID::ExtendedID(f.id()),
        }
    }
}

pub enum CanFrame {
    DataFrame(DataFrame),
    RemoteFrame(RemoteFrame),
}

impl CanFrame {
    pub fn id(&self) -> ID {
        match *self {
            CanFrame::DataFrame(ref f) => f.id(),
            CanFrame::RemoteFrame(ref f) => f.id(),
        }
    }
}


// Conversion between these types

impl From<DataFrame> for CanFrame {
    fn from(f: DataFrame) -> CanFrame {
        CanFrame::DataFrame(f)
    }
}

impl From<RemoteFrame> for CanFrame {
    fn from(f: RemoteFrame) -> CanFrame {
        CanFrame::RemoteFrame(f)
    }
}

impl From<BaseDataFrame> for DataFrame {
    fn from(f: BaseDataFrame) -> DataFrame {
        DataFrame::BaseDataFrame(f)
    }
}

impl From<ExtendedDataFrame> for DataFrame {
    fn from(f: ExtendedDataFrame) -> DataFrame {
        DataFrame::ExtendedDataFrame(f)
    }
}

impl From<BaseRemoteFrame> for RemoteFrame {
    fn from(f: BaseRemoteFrame) -> RemoteFrame {
        RemoteFrame::BaseRemoteFrame(f)
    }
}

impl From<ExtendedRemoteFrame> for RemoteFrame {
    fn from(f: ExtendedRemoteFrame) -> RemoteFrame {
        RemoteFrame::ExtendedRemoteFrame(f)
    }
}

impl From<BaseDataFrame> for CanFrame {
    fn from(f: BaseDataFrame) -> CanFrame {
        CanFrame::from(DataFrame::from(f))
    }
}

impl From<ExtendedDataFrame> for CanFrame {
    fn from(f: ExtendedDataFrame) -> CanFrame {
        CanFrame::from(DataFrame::from(f))
    }
}

impl From<BaseRemoteFrame> for CanFrame {
    fn from(f: BaseRemoteFrame) -> CanFrame {
        CanFrame::from(RemoteFrame::from(f))
    }
}

impl From<ExtendedRemoteFrame> for CanFrame {
    fn from(f: ExtendedRemoteFrame) -> CanFrame {
        CanFrame::from(RemoteFrame::from(f))
    }
}
