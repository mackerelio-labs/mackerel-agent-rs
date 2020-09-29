use serde_json::{map::Map, Value};
// use std::collections::HashMap;

// TODO:
//   - mackerel_client_rs に持って行けるか考へる
//   - Cloud の定義
//   - JSON への変換のところまともにして
pub struct HostMeta {
    pub agent_name: String,
    pub agent_revision: String,
    pub agent_version: String,
    // pub block_device: HashMap<String, HashMap<String, String>>,
    // pub cloud: Cloud,
    // pub cpu: Vec<Cpu>,
    // pub filesystem: HashMap<String, Filesystem>,
    // pub kernel: Kernel,
    // pub memory: Memory,
}

// pub struct Cloud {
// 	pub provider: String,
// 	pub meta_data: ,
// }

// pub struct Cpu {
//     pub cache_size: String,
//     pub core_id: String,
//     pub cores: String,
//     pub family: String,
//     pub mhz: String,
//     pub model: String,
//     pub model_name: String,
//     pub physical_id: String,
//     pub stepping: String,
//     pub vendor_id: String,
// }

// pub struct Filesystem {
//     pub kb_available: u64,
//     pub kb_size: u64,
//     pub kb_used: u64,
//     pub mount: String,
//     pub percent_used: String,
// }

// pub struct Kernel {
//     pub machine: String,
//     pub name: String,
//     pub os: String,
//     pub platform_name: String,
//     pub platform_version: String,
//     pub release: String,
//     pub version: String,
// }

// pub struct Memory {
//     pub active: String,
//     pub anon_pages: String,
//     pub bounce: String,
//     pub buffers: String,
//     pub cached: String,
//     pub commit_limit: String,
//     pub committed_as: String,
//     pub dirty: String,
//     pub free: String,
//     pub high_free: String,
//     pub high_total: String,
//     pub inactive: String,
//     pub low_free: String,
//     pub low_total: String,
//     pub mapped: String,
//     pub nfs_unstable: String,
//     pub page_tables: String,
//     pub slab: String,
//     pub slab_reclaimable: String,
//     pub slab_unreclaim: String,
//     pub swap_cached: String,
//     pub swap_free: String,
//     pub swap_total: String,
//     pub total: String,
//     pub vmalloc_chunk: String,
//     pub vmalloc_total: String,
//     pub vmalloc_used: String,
//     pub writeback: String,
// }

pub fn collect() -> HostMeta {
    HostMeta {
        agent_name: "mackerel-agent-rs/0.0.1 (Revision f2f87cb)".to_owned(),
        agent_revision: "f2f87cb".to_owned(),
        agent_version: "0.0.1".to_owned(),
    }
}

pub fn collect_as_json() -> Map<String, Value> {
    let meta = collect();
    let mut meta_json = Map::new();
    meta_json.insert("agent_name".to_owned(), Value::String(meta.agent_name));
    meta_json.insert(
        "agent_revision".to_owned(),
        Value::String(meta.agent_revision),
    );
    meta_json.insert(
        "agent_version".to_owned(),
        Value::String(meta.agent_version),
    );
    meta_json
}
