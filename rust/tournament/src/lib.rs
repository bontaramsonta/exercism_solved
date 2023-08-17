use std::collections::HashMap;
pub fn tally(match_results: &str) -> String {
    let mut table = vec![format!(
        "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P"
    )];
    let mut map: HashMap<&str, Vec<u32>> = HashMap::new(); // team : [MP, W, D]
    match_results.split("\n").for_each(|x| {
        // when no input
        if x == "" {
            return;
        }
        // team1;team2;result
        println!("input_line: {x}");
        let mut row = x.split(';');
        let team1 = row.next().unwrap();
        let team2 = row.next().unwrap();
        let result = row.next().unwrap();
        let (team1_result, team2_result) = match result {
            "win" => (1, 0),
            "loss" => (0, 1),
            "draw" => (0, 0),
            _ => panic!("Invalid result"),
        };
        // initialize team stats
        map.entry(team1).or_insert(vec![0, 0, 0]);
        map.entry(team2).or_insert(vec![0, 0, 0]);
        // update team stats
        // increment MP
        map.get_mut(team1).unwrap()[0] += 1;
        map.get_mut(team2).unwrap()[0] += 1;
        // increment W
        map.get_mut(team1).unwrap()[1] += team1_result;
        map.get_mut(team2).unwrap()[1] += team2_result;
        // increment D
        map.get_mut(team1).unwrap()[2] += 1 - team1_result - team2_result;
        map.get_mut(team2).unwrap()[2] += 1 - team2_result - team1_result;
    });
    let mut table_entries: Vec<Vec<String>> = map
        .iter()
        .map(|(team, stats)| {
            vec![
                team.to_string(),
                stats[0].to_string(),                         // MP
                stats[1].to_string(),                         // W
                stats[2].to_string(),                         // D
                (stats[0] - stats[1] - stats[2]).to_string(), // L
                (stats[1] * 3 + stats[2]).to_string(),        // P
            ]
        })
        .collect();
    // sort table entries by points, then by team name
    table_entries.sort_by(|a, b| {
        let a_points = a[5].parse::<u32>().unwrap();
        let b_points = b[5].parse::<u32>().unwrap();
        // if points are equal, sort by team name
        if a_points == b_points {
            a[0].cmp(&b[0])
        }
        // else sort by points
        else {
            b_points.cmp(&a_points)
        }
    });
    table.extend(table_entries.iter().map(|x| {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            x[0], x[1], x[2], x[3], x[4], x[5]
        )
    }));
    table.join("\n")
}
