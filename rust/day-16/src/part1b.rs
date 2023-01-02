use std::collections::HashMap;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::multi::separated_list0;
use nom::branch::alt;
use nom::character::complete::anychar;


#[derive(Debug)]
struct Node {
    name: String, 
    rate: i32,
    conn: Vec::<i32>
}

fn parse_input(input: &str) -> HashMap::<i32, Node> {
    let mut dicitinary: HashMap<&str, i32> = HashMap::new();
    let mut nodes: HashMap::<i32, Node> = HashMap::new();
    let mut tmp_s: String;
    for (i, line) in input.lines().enumerate() {
        dicitinary.insert(
            (delimited(tag("Valve "), alpha1, tag(" has "))(line) as IResult<&str, &str>).unwrap().1,
            i as i32
        );
    }
    
    for (i, line) in input.lines().enumerate() {
        let (mut s, name) = (delimited(tag("Valve "), alpha1, tag(" has "))(line) as IResult<&str, &str>).unwrap();
        let (mut s, rate) = (preceded(tag("flow rate="), nom::character::complete::i32)(s) as IResult<&str, i32>).unwrap();
        tmp_s = s.replace("; tunnels lead to valves ", "");
        tmp_s = tmp_s.replace("; tunnel leads to valve ", "");
        nodes.insert(i as i32,
             Node{
                name: name.to_string(),
                rate: rate,
                conn: tmp_s.split(", ").map(|s| *dicitinary.get(s).unwrap()).collect::<Vec<i32>>(),
            }
        );
    }
    nodes 
}

fn find_index_on_name(name: &str, nodes: &HashMap<i32, Node>) -> i32 {
    for (i, node) in nodes.iter() {
        if name.to_string() == node.name {
            return *i;
        }
    }
    panic!("Could not find name");
}


fn explore(node_i: i32, current_score: i32, time: i32, node_p: i32, nodes: &mut HashMap<i32, Node>) -> (i32, Vec<i32>) {
    let mut branch_score;
    let mut best_score = 0;
    let mut trail;
    let mut best_trail = Vec::<i32>::new();
    
    //println!("In Node {:?}", nodes.get(&node_i).unwrap().name);
    // if Time out
    if time == 0 {
        //println!("Timing out");
        let mut v = Vec::<i32>::new();
        v.push(node_i);
        return (current_score, v);
    }    

    // if opening
    if nodes.get(&node_i).unwrap().rate > 0 {
        //println!("Opening");
        let rate = nodes.get(&node_i).unwrap().rate.clone();
        nodes.get_mut(&node_i).unwrap().rate = 0;
        (branch_score, trail) = explore(node_i, current_score + (time-1)*rate, time-1, node_p, nodes);
        if branch_score > best_score {
            best_score = branch_score;
            best_trail = trail;
        }
        nodes.get_mut(&node_i).unwrap().rate = rate.clone();
    }

    // If walking
    let conn = nodes.get(&node_i).unwrap().conn.clone();
    for node_j in conn.iter() {
        if (conn.len() > 1 && *node_j == node_p)  {
            continue;
        } 
        (branch_score, trail)  = explore(*node_j, current_score, time-1, node_i, nodes);
        if branch_score > best_score {
            best_score = branch_score;
            best_trail = trail;
        }
    }
    best_trail.push(node_i);   
    (best_score, best_trail)
}

function Dijkstra(Graph, source):
 2      
 3      for each vertex v in Graph.Vertices:
 4          dist[v] ← INFINITY
 5          prev[v] ← UNDEFINED
 6          add v to Q
 7      dist[source] ← 0
 8      
 9      while Q is not empty:
10          u ← vertex in Q with min dist[u]
11          remove u from Q
12          
13          for each neighbor v of u still in Q:
14              alt ← dist[u] + Graph.Edges(u, v)
15              if alt < dist[v]:
16                  dist[v] ← alt
17                  prev[v] ← u
18
19      return dist[], prev[]

struct Node {
    neighbors: Vec<u32>
}

struct Graph {
    nodes: Vec<Node>
}

fn find_shortest_paths(graph, source) {
    // Q ordered lis

    for v in graph.vertices {
        dist[v] = -1;
        prev[v] = -1;
        q.push(v)
    }
    dist = 0
    while ! q.is_empty() {
        node = Q.pop();
        for adj in graph[node].adjeciant() {
            alternative = dist[node] + graph.edge(node, adj)
            if alternative < dist[adj] {
                dist[adj] = alternative;
                prev[adj] = node; 
            }
        }
    }
}

pub fn main(input: &str) -> String {
    let mut nodes = parse_input(input);
    let start_i = find_index_on_name("AA", &nodes);
    let (score, trail) = explore(start_i, 0, 30, start_i, &mut nodes);
    println!("{:?}", start_i);
    println!("{:?}", score);
    score.to_string()
}



