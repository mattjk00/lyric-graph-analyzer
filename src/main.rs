/// Rust Lyric Graph Analyzer
/// Matthew Kleitz, 2020
/// Vino Axon Technologies
extern crate rand;
extern crate regex;
use regex::Regex;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

/// Basic graph vertex. Contains a value that represents a single word lyric.
struct LVert {
    value:String   
}
impl LVert {
    fn new(val:String) -> LVert {
        LVert { value: val }
    }
}

/// An implementation of a graph structure used for lyric analysis.
/// Contains an adjacency matrix (2x2 vector of bits),
/// and a list of references to the vertices of the graph.
struct LGraph<'a> {
    matrix:Vec<Vec<u8>>,
    vertices:Vec<&'a LVert>,
    count:usize
}

impl<'a> LGraph<'a> {

    /// Creates a blank graph with an adjacency matrix of 100x100.
    /// Also inits a blank array of vertices.
    fn new(size:usize) -> LGraph<'a> {
        LGraph { 
            matrix:vec![vec![0; size]; size],
            vertices:vec![],
            count:0
        }
    }

    /// Tries to add a given vertex to the vertex vector.
    /// Will not add a duplicate vertex.
    fn add_vertex(&mut self, v:&'a LVert) {
        let index = self.find_vertex(v);
        match index {
            Some(i) => {
                println!("Vertex already in graph!");
            },
            None => {
                self.vertices.push(v);
                self.count += 1;
                println!("Vertex added!");
            }
        }
    }

    /// Adds a range of given vertices.
    /// Utilizes LGraph::add_vertex
    fn add_vertices(&mut self, vs:Vec<&'a LVert>) {
        for v in vs.iter() {
            self.add_vertex(v);
        }
    }

    /// Finds the index of the first instance of a given vertex in the graph.
    /// If the vertex is not found it returns None, otherwise it returns a usize in an Option.
    fn find_vertex(&mut self, v:&LVert) -> Option<usize> {
        let mut i:usize = 0;
        for vert in self.vertices.iter() {
            if vert.value == v.value {
                return Some(i);
            }
            i = i + 1;
        }
        None
    }

    /// Tries to create an edge between to given vertices. 
    /// If both vertices exist in the graph, the corresponding point in the adjacency matrix will be set to 1.
    fn create_edge(&mut self, v1:&LVert, v2:&LVert) {
        let index1 = self.find_vertex(v1);
        let index2 = self.find_vertex(v2);
        if index1.is_some() && index2.is_some() {
            self.matrix[index1.unwrap()][index2.unwrap()] = 1;
        } else {
            println!("Failed to create edge.");
        }
    }

    /// Traverses the graph in a psuedo-random way from a given start index to try and form a sentence.
    /// Returns a vector of strings.
    fn traverse_sentence(&mut self, start:&LVert, length:usize) -> Vec<String> {
        // check that the starting index exists
        let start_index_option = self.find_vertex(start);
        
        // to store our generated sentence
        let mut sentence = vec![];

        if start_index_option.is_some() {
            // get the start index from the option
            let mut start_index = start_index_option.unwrap();
            let mut rng = rand::thread_rng();

            // push the starting vertex value to the sentence
            sentence.push(self.vertices[start_index].value.clone());
            
            for i in 0..length - 1 {

                // create an array of possible vertex indices to visit
                let mut possible_indices = vec![];
                // iterate thru the starting vertex's adjacency array
                for n in 0..self.matrix[start_index].len() {
                    // possible vertex to check and maybe add
                    let pos_vert = self.matrix[start_index][n];
                    // if the vertex is adjacent, push it the possible indices vector
                    if pos_vert == 1 {
                        possible_indices.push(n);
                    }
                }
                
                // choose a random index from the possibilites
                let random_possible_index = rng.gen_range(0, possible_indices.len());
                // choose the next vertex index to visit
                let next_vi = possible_indices[random_possible_index];

                // push the value of the next vertex to the sentence
                sentence.push(self.vertices[next_vi].value.clone());
                // repeat the process, but using the word we just found as the starting point
                start_index = next_vi;
            }
        }
        sentence
    }

    /// Basic print out of the adjacency matrix.
    fn print(&mut self) {
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix.len() {
                print!("{}", self.matrix[j][i]);
            }
            println!();
        }
    }
}

/// Reads a text file full of lyrics and converts them into an vector of vertices.
fn read_lyric_file() -> Vec<LVert> {
    // read the text file
    let mut f = File::open("lyrics.txt").expect("Could not open file!!");
    let mut text = String::new();
    f.read_to_string(&mut text).expect("Failed to read open file!!");

    // clean up the text a  bit
    text = text.to_lowercase().replace("\n", "");

    // use a regex to remove all punctuation from the string
    let re = Regex::new(r"[^\w\s]").unwrap();
    text = re.replace_all(&text, "").to_string();

    // clean up the rest of the text and split it into a vector of strings
    let mut cleaned = text.split_whitespace().collect::<Vec<_>>();
    let mut vertices = vec![];

    // created list of vertices
    for i in 0..cleaned.len() {
        vertices.push(LVert::new(cleaned[i].to_string()));
    }

    // return the vertices
    vertices
}

fn main() {
    // create lyric vertices from the text file
    let vertices = read_lyric_file();
    // initialize the graph structure
    let mut graph = LGraph::new(vertices.len());

    // add every vertex to the graph
    for i in 0..vertices.len() {
        graph.add_vertex(&vertices[i]);
    }
    // create all the edges in the graph.
    // Any two given two pairs of subsequent words in a song will be given an adjacency value of 1
    // EX: "The cat meows cat" would be read in and create a matrix like so:
    // 010
    // 001
    // 010
    for i in 0..vertices.len() -1 {
        graph.create_edge(&vertices[i], &vertices[i + 1]);
    }
    
    println!("\n");

    // should clean this up later...
    // at the moment this will produce 5 lines of a song with sentences that are either 5 or 6 words long.
    for i in 0..5 {
        let sent = graph.traverse_sentence(&vertices[rand::thread_rng().gen_range(0, vertices.len())], 5 + i % 2);
        for word in sent.iter() {
            print!("{} ", word);
        }
        println!();
    }
}
