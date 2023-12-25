use std::collections::HashMap;
use itertools::Itertools;
use rand::seq::SliceRandom;

pub fn exec_day25_part1(input: &str) -> String {
    let mut adjacency_list: HashMap<String, (i32, Vec<String>)> = HashMap::new();
    for line in input.lines() {
        let nodes = line.split(": ").collect_vec();
        let root = nodes[0];
        let other = nodes[1].split(' ').map(|s| s.to_string()).collect_vec();
        if !adjacency_list.keys().contains(&root.to_string()) {
            adjacency_list.insert(root.to_string(), (1, Vec::new()));
        }
        adjacency_list.get_mut(root).unwrap().1.append(&mut other.clone());
        for neighbor in other {
            if !adjacency_list.keys().contains(&neighbor.to_string()) {
                adjacency_list.insert(neighbor.to_string(), (1, Vec::new()));
            }
            adjacency_list.get_mut(&neighbor).unwrap().1.push(root.to_string());
        }
    }
    let mut rng = rand::thread_rng();
    let result = loop {
        let mut g = adjacency_list.clone();
        while g.keys().len() > 2 {
            let keys = g.keys().cloned().collect_vec();
            let u = keys.choose(&mut rng).unwrap();
            let v = &g[&u.to_string()].1.choose(&mut rng).unwrap().to_string();
            let uv = format!("{v}{u}");
            let mut u_entry = g.remove(&u.to_string()).unwrap();
            let mut v_entry = g.remove(&v.to_string()).unwrap();
            u_entry.1.append(&mut v_entry.1);
            u_entry.1.retain(|node| node != u && node != v);
            for node in &u_entry.1 {
                g.get_mut(node).unwrap().1 = g[&node.to_string()].1.iter().map(|n| if n == u || n == v { uv.to_string() } else { n.to_string() }).collect_vec();
            }
            g.insert(uv, (u_entry.0 + v_entry.0, u_entry.1));
        }
        let mut keys = g.keys();
        let u = keys.next().unwrap();
        let v = keys.next().unwrap();
        if g[u].1.len() == 3 {
            break g[v].0 * g[u].0
        }
    };
    result.to_string()
}

pub fn exec_day25_part2(input: &str) -> String {
    3.to_string()
}
