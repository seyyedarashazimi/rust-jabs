use crate::ledger_data::block::Block;
use crate::ledger_data::block_factory::{
    BITCOIN_INV_SIZE, GET_DATA_OVERHEAD, INV_MESSAGE_OVERHEAD,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum DataType {
    IsBlock,
    IsTx,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum MessageType {
    DataMessage(DataType),
    InvMessage(DataType),
    RequestDataMessage(DataType),
    VoteMessage,
}

impl MessageType {
    pub fn get_size(&self, block_index: usize, blocks: &[Block]) -> u64 {
        match self {
            Self::DataMessage(dt) => Self::bitcoin_data_size(dt, block_index, blocks),
            Self::InvMessage(_) => Self::bitcoin_inv_size(),
            Self::RequestDataMessage(_) => Self::bitcoin_request_data_size(),
            Self::VoteMessage => u64::default(), // never should be used for bitcoin.
        }
    }

    fn bitcoin_data_size(data_type: &DataType, block_index: usize, blocks: &[Block]) -> u64 {
        match data_type {
            DataType::IsBlock => blocks[block_index].size,
            DataType::IsTx => 1_u64, // todo
        }
    }

    fn bitcoin_inv_size() -> u64 {
        BITCOIN_INV_SIZE + INV_MESSAGE_OVERHEAD
    }

    fn bitcoin_request_data_size() -> u64 {
        BITCOIN_INV_SIZE + GET_DATA_OVERHEAD
    }
}
