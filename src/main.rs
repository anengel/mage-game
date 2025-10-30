use std::collections::{VecDeque, HashSet};
use std::fmt;

// 迷宫单元格类型
#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,    // 空地
    Wall,     // 墙
    Start,    // 起点
    End,      // 终点
    Path,     // 路径标记
    Player,   // 玩家
}

// 方向枚举
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 位置结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

// 迷宫游戏结构体
struct MazeGame {
    grid: Vec<Vec<Cell>>,
    player_pos: Position,
    start_pos: Position,
    end_pos: Position,
    width: usize,
    height: usize,
}

impl MazeGame {
    fn new(width: usize, height: usize) -> Self {
        let mut grid = vec![vec![Cell::Empty; width]; height];
        
        // 设置边界墙
        for i in 0..height {
            grid[i][0] = Cell::Wall;
            grid[i][width - 1] = Cell::Wall;
        }
        for j in 0..width {
            grid[0][j] = Cell::Wall;
            grid[height - 1][j] = Cell::Wall;
        }
        
        // 添加一些内部墙
        for i in 2..height-2 {
            if i % 3 == 0 {
                for j in 2..width-2 {
                    if j % 4 == 0 {
                        grid[i][j] = Cell::Wall;
                    }
                }
            }
        }
        
        // 设置起点和终点
        let start_pos = Position { x: 1, y: 1 };
        let end_pos = Position { x: width - 2, y: height - 2 };
        
        grid[start_pos.y][start_pos.x] = Cell::Start;
        grid[end_pos.y][end_pos.x] = Cell::End;
        
        let mut game = MazeGame {
            grid,
            player_pos: start_pos,
            start_pos,
            end_pos,
            width,
            height,
        };
        
        game.update_player_position(start_pos);
        game
    }
    
    // 更新玩家位置
    fn update_player_position(&mut self, new_pos: Position) {
        // 清除旧位置（如果是起点则恢复为起点，否则恢复为空地）
        if self.player_pos == self.start_pos {
            self.grid[self.player_pos.y][self.player_pos.x] = Cell::Start;
        } else {
            self.grid[self.player_pos.y][self.player_pos.x] = Cell::Empty;
        }
        
        // 设置新位置
        self.player_pos = new_pos;
        self.grid[new_pos.y][new_pos.x] = Cell::Player;
    }
    
    // 碰撞检测
    fn can_move(&self, pos: Position) -> bool {
        if pos.x >= self.width || pos.y >= self.height {
            return false;
        }
        
        match self.grid[pos.y][pos.x] {
            Cell::Wall => false,
            _ => true,
        }
    }
    
    // 移动玩家
    fn move_player(&mut self, direction: Direction) -> bool {
        let new_pos = match direction {
            Direction::Up if self.player_pos.y > 0 => {
                Position { x: self.player_pos.x, y: self.player_pos.y - 1 }
            }
            Direction::Down => {
                Position { x: self.player_pos.x, y: self.player_pos.y + 1 }
            }
            Direction::Left if self.player_pos.x > 0 => {
                Position { x: self.player_pos.x - 1, y: self.player_pos.y }
            }
            Direction::Right => {
                Position { x: self.player_pos.x + 1, y: self.player_pos.y }
            }
            _ => return false,
        };
        
        if self.can_move(new_pos) {
            self.update_player_position(new_pos);
            true
        } else {
            false
        }
    }
    
    // 检查是否获胜
    fn has_won(&self) -> bool {
        self.player_pos == self.end_pos
    }
    
    // 使用BFS寻找最短路径
    fn find_shortest_path(&self) -> Option<Vec<Position>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = vec![vec![None; self.width]; self.height];
        
        queue.push_back(self.start_pos);
        visited.insert(self.start_pos);
        
        while let Some(current) = queue.pop_front() {
            if current == self.end_pos {
                // 重建路径
                let mut path = Vec::new();
                let mut step = current;
                
                while step != self.start_pos {
                    path.push(step);
                    step = parent[step.y][step.x].unwrap();
                }
                path.reverse();
                return Some(path);
            }
            
            // 检查四个方向
            let directions = [
                (0, -1), // 上
                (0, 1),  // 下
                (-1, 0), // 左
                (1, 0),  // 右
            ];
            
            for &(dx, dy) in &directions {
                let x = current.x as i32 + dx;
                let y = current.y as i32 + dy;
                
                if x >= 0 && y >= 0 {
                    let new_pos = Position { 
                        x: x as usize, 
                        y: y as usize 
                    };
                    
                    if new_pos.x < self.width && new_pos.y < self.height 
                        && self.can_move(new_pos) 
                        && !visited.contains(&new_pos) {
                        
                        visited.insert(new_pos);
                        parent[new_pos.y][new_pos.x] = Some(current);
                        queue.push_back(new_pos);
                    }
                }
            }
        }
        
        None
    }
    
    // 显示路径
    fn display_path(&mut self) {
        if let Some(path) = self.find_shortest_path() {
            for pos in path {
                if self.grid[pos.y][pos.x] == Cell::Empty {
                    self.grid[pos.y][pos.x] = Cell::Path;
                }
            }
        }
    }
    
    // 清除路径显示
    fn clear_path(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.grid[i][j] == Cell::Path {
                    self.grid[i][j] = Cell::Empty;
                }
            }
        }
    }
}

impl fmt::Display for MazeGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => " ",
                    Cell::Wall => "█",
                    Cell::Start => "S",
                    Cell::End => "E",
                    Cell::Path => "·",
                    Cell::Player => "P",
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// 游戏控制函数
fn play_game() {
    let mut game = MazeGame::new(15, 10);
    
    println!("=== 迷宫游戏 ===");
    println!("使用 WASD 移动玩家(P)");
    println!("从起点(S)移动到终点(E)");
    println!("输入 'path' 显示最短路径");
    println!("输入 'clear' 清除路径");
    println!("输入 'quit' 退出游戏");
    println!();
    
    let mut input = String::new();
    
    loop {
        println!("{}", game);
        
        if game.has_won() {
            println!("恭喜！你赢了！");
            break;
        }
        
        println!("请输入命令 (WASD/path/clear/quit): ");
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();
        
        match command.as_str() {
            "w" => { game.move_player(Direction::Up); }
            "s" => { game.move_player(Direction::Down); }
            "a" => { game.move_player(Direction::Left); }
            "d" => { game.move_player(Direction::Right); }
            "path" => { 
                game.display_path();
                println!("已显示最短路径");
            }
            "clear" => { 
                game.clear_path();
                println!("已清除路径显示");
            }
            "quit" => {
                println!("游戏结束！");
                break;
            }
            _ => println!("无效命令！"),
        }
        
        println!();
    }
}

fn main() {
    play_game();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation() {
        let game = MazeGame::new(10, 10);
        assert_eq!(game.grid[1][1], Cell::Player); // 玩家在起点
        assert_eq!(game.grid[8][8], Cell::End);    // 终点在右下角
    }

    #[test]
    fn test_collision_detection() {
        let game = MazeGame::new(10, 10);
        // 测试墙碰撞
        assert!(!game.can_move(Position { x: 0, y: 0 })); // 角落墙
        // 测试空地移动
        assert!(game.can_move(Position { x: 1, y: 2 })); // 空地
    }

    #[test]
    fn test_path_finding() {
        let game = MazeGame::new(10, 10);
        let path = game.find_shortest_path();
        assert!(path.is_some(), "应该能找到路径");
    }
}