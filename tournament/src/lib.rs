use std::collections::HashMap;

/// 团队统计信息
struct TeamStats {
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
}

impl TeamStats {
    fn new() -> Self {
        TeamStats {
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    /// 计算积分：胜3分，平1分，负0分
    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

/// 生成联赛积分榜
///
/// # 输入格式
/// 每行格式：`Team1;Team2;result`
/// - result 可以是 "win"（Team1 赢）、"loss"（Team1 输）、"draw"（平局）
///
/// # 输出格式
/// 表格包含：Team | MP | W | D | L | P
/// - 按积分降序排序，积分相同时按团队名字母序排序
pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, TeamStats> = HashMap::new();

    // 解析每场比赛结果
    for line in match_results.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue;
        }

        let team1 = parts[0].to_string();
        let team2 = parts[1].to_string();
        let result = parts[2];

        // 确保两个团队都存在
        teams.entry(team1.clone()).or_insert_with(TeamStats::new);
        teams.entry(team2.clone()).or_insert_with(TeamStats::new);

        // 更新统计信息（分别获取引用）
        {
            let stats1 = teams.get_mut(&team1).unwrap();
            stats1.matches_played += 1;
            match result {
                "win" => stats1.wins += 1,
                "loss" => stats1.losses += 1,
                "draw" => stats1.draws += 1,
                _ => continue,
            }
        }

        {
            let stats2 = teams.get_mut(&team2).unwrap();
            stats2.matches_played += 1;
            match result {
                "win" => stats2.losses += 1,
                "loss" => stats2.wins += 1,
                "draw" => stats2.draws += 1,
                _ => {}
            }
        }
    }

    // 转换为向量并排序
    let mut team_vec: Vec<(String, TeamStats)> = teams.into_iter().collect();
    team_vec.sort_by(|a, b| {
        // 首先按积分降序排序
        let points_cmp = b.1.points().cmp(&a.1.points());
        if points_cmp != std::cmp::Ordering::Equal {
            points_cmp
        } else {
            // 积分相同时按团队名字母序排序
            a.0.cmp(&b.0)
        }
    });

    // 生成表格
    let mut result = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];

    for (team_name, stats) in team_vec {
        let line = format!(
            "{:<30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            team_name,
            stats.matches_played,
            stats.wins,
            stats.draws,
            stats.losses,
            stats.points()
        );
        result.push(line);
    }

    result.join("\n")
}
