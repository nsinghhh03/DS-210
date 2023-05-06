use std::collections::HashMap;
use std::error::Error;
use csv::Reader;

#[allow(dead_code)]
#[derive(Default)]
struct Node {  
    food_insecurity_score: f64,
    census_tract: String,
    county: String,
    urban: String,
    pop_2010: String,
    housing_units: String,
    group_quarters: String,
    vehicle_access: String,
    low_income: String,
    poverty_rate: String,
    median_family_income: String,
    no_supermarket: String, 
    tract_kids: String,
    tract_seniors: String,
    tract_white: String,
    tract_black: String,
    tract_asian: String,
    tract_hispanic: String,
    tract_snap: String,
    edges: Vec<String>,
}



fn read_csv(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let data: Vec<Vec<String>> = reader
        .records()
        .filter_map(Result::ok)
        .map(|record| record.iter().map(|field| field.to_owned()).collect())
        .collect();

    println!("Finished reading CSV file: {}", file_path);

    Ok(data)
}


fn should_add_edge(node: &Node, other_node: &Node) -> bool {
    if node.census_tract == other_node.census_tract {
        return false;
    }

    // Check if nodes share the same county and urban/rural classification.
    if node.county == other_node.county && node.urban == other_node.urban {
        return true;
    }
              
    // Check if nodes are nearly adjacent based on tract ID
    let node_id: Option<i32> = node.census_tract.chars().skip(5).collect::<String>().parse().ok();
    let other_node_id: Option<i32> = other_node.census_tract.chars().skip(5).collect::<String>().parse().ok();

    if let (Some(node_id), Some(other_node_id)) = (node_id, other_node_id) {
        if (node_id - other_node_id).abs() <= 10 {
            return true;
        }
    }

    false
}



fn create_nodes(data: Vec<Vec<String>>) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for record in data {
        let census_tract = &record[0];

        let poverty_rate: f64 = record[8].parse().unwrap();
        let no_supermarket: f64 = record[10].parse().unwrap();
        let tract_snap: f64 = record[17].parse().unwrap();
        let food_insecurity_score =  poverty_rate + no_supermarket + tract_snap;

        let node = Node {
            food_insecurity_score,
            census_tract: census_tract.to_owned(),
            county: record[1].to_owned(),
            urban: record[2].to_owned(),
            pop_2010: record[3].to_owned(),
            housing_units: record[4].to_owned(),
            vehicle_access: record[5].to_owned(),
            group_quarters: record[6].to_owned(),
            low_income: record[7].to_owned(),
            poverty_rate: record[8].to_owned(),
            median_family_income: record[9].to_owned(),
            no_supermarket: record[10].to_owned(),
            tract_kids: record[11].to_owned(),
            tract_seniors: record[12].to_owned(),
            tract_white: record[13].to_owned(),
            tract_black: record[14].to_owned(),
            tract_asian: record[15].to_owned(),
            tract_hispanic: record[16].to_owned(),
            tract_snap: record[17].to_owned(),
            edges: Vec::new(),
        };
        nodes.insert(census_tract.to_owned(), node);
    }

    nodes
} 

fn create_edges(nodes: &mut HashMap<String, Node>) {
    for i in 0..nodes.len() {
        let node_key = nodes.keys().nth(i).unwrap().to_owned();
        let node = nodes.get(&node_key).unwrap();
        let mut edges_to_add = Vec::new();

        for j in (i + 1)..nodes.len() {
            let other_node_key = nodes.keys().nth(j).unwrap().to_owned();
            let other_node = nodes.get(&other_node_key).unwrap();

            if should_add_edge(node, other_node) {
                edges_to_add.push((node_key.clone(), other_node_key.clone()));
            }
        }

        for (node_key, other_node_key) in edges_to_add {
            if let Some(node_mut) = nodes.get_mut(&node_key) {
                node_mut.edges.push(other_node_key.clone());
            }
            if let Some(other_node_mut) = nodes.get_mut(&other_node_key) {
                other_node_mut.edges.push(node_key.clone());
            }
        }
    }
}


fn calculate_degree_centrality(nodes: &HashMap<String, Node>, num_vertices: usize) -> (f64, Option<&Node>) {
    let mut max_degree_centrality = 0.0;
    let mut max_food_insecurity_score = 0.0;
    let mut max_degree_centrality_node: Option<&Node> = None;

    for (_, node) in nodes {
        let degree_centrality = node.edges.len() as f64 / (num_vertices - 1) as f64;
        if node.food_insecurity_score > max_food_insecurity_score {
            max_food_insecurity_score = node.food_insecurity_score;
            max_degree_centrality = degree_centrality;
            max_degree_centrality_node = Some(node);
        } else if node.food_insecurity_score == max_food_insecurity_score && degree_centrality > max_degree_centrality {
            max_degree_centrality = degree_centrality;
            max_degree_centrality_node = Some(node);
        }
    }

    (max_degree_centrality, max_degree_centrality_node)
}



fn main() {
    println!("Starting the program...");
    let file_path = "/Users/neeza/Downloads/NYFoodAccessData.csv";
    let data = read_csv(file_path).unwrap();
    let num_vertices = data.len();
    println!("Number of Vertices: {}", num_vertices);
    

    // Create a HashMap to store the nodes
    let mut nodes = create_nodes(data);

    // Create edges
    create_edges(&mut nodes);

    // Print the nodes and their edges
    for (key, node) in &nodes {
        println!("Node: {}, Edges: {:?}", key, node.edges);
    }

    // Calculate the degree centrality for each node
    let (max_degree_centrality, max_degree_centrality_node) = calculate_degree_centrality(&nodes, num_vertices);

    // Print the node with the highest degree centrality
    if let Some(node) = max_degree_centrality_node {
        println!(
            "Node with the highest degree centrality: {}nCounty: {}\nDegree Centrality: {}\nFood Insecurity Score: {}",
            node.census_tract, node.county, max_degree_centrality, node.food_insecurity_score
        );
    } else {
        println!("No nodes found.");
    }
}


//BEGINNING OF TEST MODULE
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_read_csv() {
        let file_path = "/Users/neeza/Downloads/NYFoodAccessData.csv";
        let data = read_csv(file_path).unwrap();
        assert_eq!(data.len(), 1616); // Num of rows in CSV file
    }

    //Using adjacent census tracts 
    #[test]
    fn test_should_add_edge() {
        let node1 = Node {
            food_insecurity_score: 15.7,
            census_tract: "CT1".to_string(),
            county: "Albany County".to_string(),
            urban: "1".to_string(),
            pop_2010: "101".to_string(),
            housing_units: "53".to_string(),
            group_quarters: "0".to_string(),
            vehicle_access: "1".to_string(),
            low_income: "1".to_string(),
            poverty_rate: "15.8".to_string(),
            median_family_income: "54000".to_string(),
            no_supermarket: "4300.0".to_string(),
            tract_kids: "34".to_string(),
            tract_seniors: "23".to_string(),
            tract_white: "53".to_string(),
            tract_black: "31".to_string(),
            tract_asian: "12".to_string(),
            tract_hispanic: "22".to_string(),
            tract_snap: "12".to_string(),
            edges: Vec::new(),
        };

        let node2 = Node {
            food_insecurity_score: 12.3,
            census_tract: "CT2".to_string(),
            county: "Albany County".to_string(),
            urban: "1".to_string(),
            pop_2010: "202".to_string(),
            housing_units: "84".to_string(),
            group_quarters: "0".to_string(),
            vehicle_access: "1".to_string(),
            low_income: "1".to_string(),
            poverty_rate: "13.5".to_string(),
            median_family_income: "58000".to_string(),
            no_supermarket: "5200.0".to_string(),
            tract_kids: "54".to_string(),
            tract_seniors: "35".to_string(),
            tract_white: "63".to_string(),
            tract_black: "24".to_string(),
            tract_asian: "7".to_string(),
            tract_hispanic: "28".to_string(),
            tract_snap: "8".to_string(),
            edges: Vec::new(),
        };

        assert_eq!(should_add_edge(&node1, &node2), true);
    }
    
    #[test]
    fn test_create_edges() {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let node1 = Node {
            census_tract: "1".to_owned(),
            county: "New York".to_owned(),
            urban: "1".to_owned(),
            pop_2010: "1000".to_owned(),
            housing_units: "500".to_owned(),
            vehicle_access: "50%".to_owned(),
            group_quarters: "200".to_owned(),
            low_income: "2347".to_owned(),
            poverty_rate: "0.10".to_owned(),
            median_family_income: "80000".to_owned(),
            no_supermarket: "Yes".to_owned(),
            tract_kids: "400".to_owned(),
            tract_seniors: "150".to_owned(),
            tract_white: "400".to_owned(),
            tract_black: "300".to_owned(),
            tract_asian: "100".to_owned(),
            tract_hispanic: "200".to_owned(),
            tract_snap: "300".to_owned(),
            food_insecurity_score: 0.0,
            edges: Vec::new(),
        };
        nodes.insert("1".to_owned(), node1);
    
        let node2 = Node {
            census_tract: "2".to_owned(),
            county: "New York".to_owned(),
            urban: "1".to_owned(),
            pop_2010: "2500".to_owned(),
            housing_units: "250".to_owned(),
            vehicle_access: "1".to_owned(),
            group_quarters: "100".to_owned(),
            low_income: "10%".to_owned(),
            poverty_rate: "0.08".to_owned(),
            median_family_income: "120000".to_owned(),
            no_supermarket: "1492".to_owned(),
            tract_kids: "200".to_owned(),
            tract_seniors: "50".to_owned(),
            tract_white: "70".to_owned(),
            tract_black: "100".to_owned(),
            tract_asian: "150".to_owned(),
            tract_hispanic: "500".to_owned(),
            tract_snap: "100".to_owned(),
            food_insecurity_score: 0.0,
            edges: Vec::new(),
        };
        nodes.insert("2".to_owned(), node2);
    
        create_edges(&mut nodes);
        
        // Node 1 should have an edge to Node 2 and vice versa
        assert_eq!(nodes.get("1").unwrap().edges.len(), 1);
        assert_eq!(nodes.get("2").unwrap().edges.len(), 1);
        assert_eq!(nodes.get("1").unwrap().edges[0], "2");
        assert_eq!(nodes.get("2").unwrap().edges[0], "1");
    }

}






    



        





            
                
                
 
    
