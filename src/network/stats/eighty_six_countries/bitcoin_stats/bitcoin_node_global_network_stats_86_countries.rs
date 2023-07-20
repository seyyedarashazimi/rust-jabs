use crate::network::stats::eighty_six_countries::{get_country_from_usize, Country};
use crate::simulator::randomness_engine::RandomnessEngine;

const BITCOIN_COUNTRIES_DISTRIBUTION_2022: [f64; 86] = [
    0.0,
    0.0,
    0.00211889346674514,
    0.0160094173042966,
    0.00494408475573867,
    0.0,
    0.00470865214832254,
    0.00965273690406121,
    0.00447321954090642,
    0.000235432607416127,
    0.0397881106533255,
    0.000824014125956445,
    0.0104767510300177,
    0.000706297822248381,
    0.000353148911124191,
    0.00176574455562095,
    0.000588581518540318,
    0.0104767510300177,
    0.00164802825191289,
    0.000588581518540318,
    0.0281341965862272,
    0.0588581518540318,
    0.0,
    0.171630370806357,
    0.0,
    0.00105944673337257,
    0.000117716303708064,
    0.000235432607416127,
    0.0103590347263096,
    0.00317834020011772,
    0.00729841082989994,
    0.00153031194820483,
    0.000588581518540318,
    0.00835785756327251,
    0.000706297822248381,
    0.00835785756327251,
    0.0164802825191289,
    0.000941730429664509,
    0.0,
    0.0,
    0.000941730429664509,
    0.000117716303708064,
    0.00565038257798705,
    0.000588581518540318,
    0.0012948793407887,
    0.000353148911124191,
    0.00188346085932902,
    0.00105944673337257,
    0.0454384932313125,
    0.00223660977045321,
    0.0,
    0.0,
    0.00270747498528546,
    0.0,
    0.000353148911124191,
    0.0,
    0.000117716303708064,
    0.00506180105944673,
    0.00235432607416127,
    0.00447321954090642,
    0.0226015303119482,
    0.000117716303708064,
    0.000824014125956445,
    0.0189523248969982,
    0.00235432607416127,
    0.00141259564449676,
    0.00353148911124191,
    0.0,
    0.00706297822248381,
    0.000117716303708064,
    0.0100058858151854,
    0.0168334314302531,
    0.00306062389640965,
    0.000117716303708064,
    0.00188346085932902,
    0.00141259564449676,
    0.0,
    0.0,
    0.00835785756327251,
    0.000588581518540318,
    0.0303708063566804,
    0.241671571512654,
    0.000117716303708064,
    0.000470865214832254,
    0.00117716303708064,
    0.125132430841672,
];

pub const BITCOIN_NUM_NODES_2022: usize = 7983;

pub struct BitcoinNodeGlobalNetworkStats86Countries {}

impl BitcoinNodeGlobalNetworkStats86Countries {
    pub fn sample_region(rand: &mut RandomnessEngine) -> Country {
        get_country_from_usize(rand.sample_from_distribution(&BITCOIN_COUNTRIES_DISTRIBUTION_2022))
    }
}