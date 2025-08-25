use congee::{CongeeCompactSet, CongeeSet};
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use shumai::{ShumaiBench, config};
use std::fmt::Display;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Serialize, Clone, Copy, Debug, Deserialize)]
pub enum Workload {
    ReadOnly,
}

impl Display for Workload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Clone, Copy, Debug, Deserialize)]
pub enum IndexType {
    CongeeSet,
    CongeeCompactSet,
}

impl Display for IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[config(path = "bench/benchmark.toml")]
pub struct CompactComparison {
    pub name: String,
    pub threads: Vec<usize>,
    pub time: usize,
    #[matrix]
    pub workload: Workload,
    #[matrix]
    pub index_type: IndexType,
    #[matrix]
    pub dataset_size: usize,
}

struct TestBench<Index: DBIndex> {
    index: Index,
    initial_cnt: usize,
    _data: Option<Vec<u8>>, // To hold the data for compact set
}

trait DBIndex: Send + Sync {
    type Guard<'a>
    where
        Self: 'a;

    fn pin<'a>(&'a self) -> Self::Guard<'a>;
    fn contains<'a>(&'a self, key: &usize, guard: &Self::Guard<'a>) -> bool;
}

impl DBIndex for CongeeSet<usize> {
    type Guard<'a> = crossbeam_epoch::Guard;

    fn pin(&self) -> Self::Guard<'_> {
        self.pin()
    }

    fn contains(&self, key: &usize, guard: &Self::Guard<'_>) -> bool {
        self.contains(key, guard)
    }
}

impl<'a> DBIndex for CongeeCompactSet<'a, usize> {
    type Guard<'b> = () where Self: 'b;

    fn pin<'b>(&'b self) -> Self::Guard<'b> {
        ()
    }

    fn contains<'b>(&'b self, key: &usize, _guard: &Self::Guard<'b>) -> bool {
        self.contains(key)
    }
}

impl<Index: DBIndex> ShumaiBench for TestBench<Index> {
    type Config = CompactComparison;
    type Result = usize;

    fn load(&mut self) -> Option<serde_json::Value> {
        // For CongeeCompactSet, we can't load data here since it's read-only
        // Data must be pre-built from an existing CongeeSet
        None
    }

    fn run(&self, context: shumai::Context<Self::Config>) -> Self::Result {
        let mut op_cnt = 0;
        let mut rng = thread_rng();

        context.wait_for_start();

        

        let guard = self.index.pin();
        while context.is_running() {
            match context.config.workload {
                Workload::ReadOnly => {
                    let key = rng.gen_range(1..self.initial_cnt);
                    match self.index.contains(&key, &guard) {
                        true => assert_eq!(key % 2, 1),
                        false => assert_eq!(key % 2, 0),
                    }
                }
            }

            op_cnt += 1;
        }
        op_cnt
    }

    fn cleanup(&mut self) -> Option<serde_json::Value> {
        None
    }
}

fn hash_key(key: usize) -> usize {
    const MULTIPLIER: usize = 0x9e3779b97f4a7c15;
    key.wrapping_mul(MULTIPLIER)
}

fn main() {
    let config = CompactComparison::load().expect("Failed to parse config!");
    let repeat = 3;

    for c in config.iter() {
        let initial_cnt = c.dataset_size;
        
        match c.index_type {
            IndexType::CongeeSet => {
                // Build a regular CongeeSet with data
                let congee_set = CongeeSet::<usize>::default();
                let guard = congee_set.pin();
                
                // Load data into CongeeSet
                for i in 0..initial_cnt {
                    if i % 2 == 0 {
                        congee_set.insert(hash_key(i), &guard).unwrap();
                    } else {
                        congee_set.insert(i, &guard).unwrap();
                    }
                }

                println!("Stats: \n{}", congee_set.stats());
                let mut test_bench = TestBench {
                    index: congee_set,
                    initial_cnt,
                    _data: None,
                };
                let result = shumai::run(&mut test_bench, c, repeat);
                result.write_json().unwrap();
            }
            IndexType::CongeeCompactSet => {
                // First build a regular CongeeSet  
                let congee_set = CongeeSet::<usize>::default();
                let guard = congee_set.pin();
                
                // Load data into CongeeSet
                for i in 0..initial_cnt {
                    if i % 2 == 0 {
                        congee_set.insert(hash_key(i), &guard).unwrap();
                    } else {
                        congee_set.insert(i, &guard).unwrap();
                    }
                }

                // Convert to compact set
                let bytes = congee_set.to_compact_set();
                let leaked_bytes: &'static [u8] = Box::leak(bytes.clone().into_boxed_slice());
                let compact_set = CongeeCompactSet::<usize>::new(leaked_bytes);

                println!("Stats: \n{}", compact_set.stats());
                println!("Memory usage: \n{}", compact_set.total_memory_bytes());

                let mut test_bench = TestBench {
                    index: compact_set,
                    initial_cnt,
                    _data: Some(bytes),
                };
                let result = shumai::run(&mut test_bench, c, repeat);
                result.write_json().unwrap();
            }
        }
    }
}