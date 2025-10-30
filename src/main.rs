use macroquad::prelude::*;
use std::collections::{VecDeque, HashSet};

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
    show_path: bool,
    path_positions: Vec<Position>,
    game_won: bool,
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
        
        // 创建一个有解的复杂迷宫
        // 使用预定义的迷宫布局
        let maze_layout = [
            "####################",
            "#S                 #",
            "# ##### ##### ##### #",
            "# #   # #   # #   # #",
            "# # ### # ### # ### #",
            "# #   # #   # #   # #",
            "# ##### ##### ##### #",
            "# #   #     #     # #",
            "# # # ##### # ##### #",
            "# # #     # #     # #",
            "# # ##### # ##### # #",
            "# #     # #     # # #",
            "# ##### # ##### # # #",
            "#       #       #   E#",
            "####################",
        ];
        
        // 将字符串布局转换为网格
        for (y, line) in maze_layout.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if y < height && x < width {
                    grid[y][x] = match ch {
                        '#' => Cell::Wall,
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        _ => Cell::Empty,
                    };
                }
            }
        }
        
        // 设置起点和终点位置
        let start_pos = Position { x: 1, y: 1 };
        let end_pos = Position { x: width - 2, y: height - 2 };
        
        // 确保起点和终点位置正确
        grid[start_pos.y][start_pos.x] = Cell::Start;
        grid[end_pos.y][end_pos.x] = Cell::End;
        
        let mut game = MazeGame {
            grid,
            player_pos: start_pos,
            start_pos,
            end_pos,
            width,
            height,
            show_path: false,
            path_positions: Vec::new(),
            game_won: false,
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
        
        // 检查是否获胜
        if self.player_pos == self.end_pos {
            self.game_won = true;
        }
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
    fn move_player(&mut self, dx: i32, dy: i32) -> bool {
        if self.game_won {
            return false;
        }
        
        let new_x = self.player_pos.x as i32 + dx;
        let new_y = self.player_pos.y as i32 + dy;
        
        if new_x >= 0 && new_y >= 0 {
            let new_pos = Position { 
                x: new_x as usize, 
                y: new_y as usize 
            };
            
            if new_pos.x < self.width && new_pos.y < self.height && self.can_move(new_pos) {
                self.update_player_position(new_pos);
                return true;
            }
        }
        false
    }
    
    // 检查是否获胜
    fn has_won(&self) -> bool {
        self.game_won
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
            self.path_positions = path;
            self.show_path = true;
        }
    }
    
    // 清除路径显示
    fn clear_path(&mut self) {
        self.show_path = false;
        self.path_positions.clear();
    }
    
    // 切换路径显示
    fn toggle_path(&mut self) {
        if self.show_path {
            self.clear_path();
        } else {
            self.display_path();
        }
    }
    
    // 重置游戏
    fn reset_game(&mut self) {
        *self = MazeGame::new(self.width, self.height);
    }
    
    // 渲染游戏
    fn render(&self, font: Option<&Font>) {
        const CELL_SIZE: f32 = 30.0;
        
        // 绘制网格
        for y in 0..self.height {
            for x in 0..self.width {
                let pos_x = x as f32 * CELL_SIZE;
                let pos_y = y as f32 * CELL_SIZE;
                
                // 跳过玩家位置，稍后单独绘制
                if self.grid[y][x] == Cell::Player {
                    continue;
                }
                
                let color = match self.grid[y][x] {
                    Cell::Wall => DARKGRAY,
                    Cell::Empty => LIGHTGRAY,
                    Cell::Start => GREEN,
                    Cell::End => RED,
                    Cell::Path => LIGHTGRAY,
                    Cell::Player => BLUE,
                };
                
                draw_rectangle(pos_x, pos_y, CELL_SIZE, CELL_SIZE, color);
                
                // 绘制网格线
                draw_rectangle_lines(pos_x, pos_y, CELL_SIZE, CELL_SIZE, 1.0, BLACK);
            }
        }
        
        // 绘制路径
        if self.show_path {
            for &pos in &self.path_positions {
                // 跳过玩家所在的位置，避免覆盖玩家
                if pos == self.player_pos {
                    continue;
                }
                let pos_x = pos.x as f32 * CELL_SIZE;
                let pos_y = pos.y as f32 * CELL_SIZE;
                draw_rectangle(pos_x, pos_y, CELL_SIZE, CELL_SIZE, YELLOW);
            }
        }
        
        // 最后绘制玩家，确保它在最上层
        let player_pos_x = self.player_pos.x as f32 * CELL_SIZE;
        let player_pos_y = self.player_pos.y as f32 * CELL_SIZE;
        draw_rectangle(player_pos_x, player_pos_y, CELL_SIZE, CELL_SIZE, BLUE);
        
        // 绘制文本说明
        let instructions = [
            "Use WASD to move",
            "Press P to show/hide path",
            "Press R to reset game",
        ];
        
        for (i, instruction) in instructions.iter().enumerate() {
            if let Some(font) = font {
                draw_text_ex(
                    instruction,
                    10.0,
                    (self.height as f32 * CELL_SIZE) + 30.0 + (i as f32 * 25.0),
                    TextParams {
                        font: Some(font),
                        font_size: 20,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(
                    instruction,
                    10.0,
                    (self.height as f32 * CELL_SIZE) + 30.0 + (i as f32 * 25.0),
                    20.0,
                    BLACK,
                );
            }
        }
        
        if self.game_won {
            let win_message = "Congratulations! You won! Press R to restart";
            if let Some(font) = font {
                draw_text_ex(
                    win_message,
                    10.0,
                    (self.height as f32 * CELL_SIZE) + 30.0 + (3 as f32 * 25.0),
                    TextParams {
                        font: Some(font),
                        font_size: 20,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(
                    win_message,
                    10.0,
                    (self.height as f32 * CELL_SIZE) + 30.0 + (3 as f32 * 25.0),
                    20.0,
                    BLACK,
                );
            }
        }
    }
}

#[macroquad::main("Maze Game")]
async fn main() {
    let width = 20;
    let height = 15;
    
    let mut game = MazeGame::new(width, height);
    
    // 尝试加载字体
    let font = load_ttf_font("assets/FiraSans-Regular.ttf").await.ok();
    
    loop {
        clear_background(WHITE);
        
        // 处理输入
        if is_key_pressed(KeyCode::P) {
            game.toggle_path();
        }
        
        if is_key_pressed(KeyCode::R) {
            game.reset_game();
        }
        
        // 按键移动处理
        if is_key_pressed(KeyCode::W) {
            game.move_player(0, -1);
        }
        if is_key_pressed(KeyCode::S) {
            game.move_player(0, 1);
        }
        if is_key_pressed(KeyCode::A) {
            game.move_player(-1, 0);
        }
        if is_key_pressed(KeyCode::D) {
            game.move_player(1, 0);
        }
        
        // 渲染游戏
        game.render(font.as_ref());
        
        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation() {
        let game = MazeGame::new(20, 15);
        assert_eq!(game.grid[1][1], Cell::Player);
        assert_eq!(game.grid[13][18], Cell::End);
    }

    #[test]
    fn test_collision_detection() {
        let game = MazeGame::new(20, 15);
        assert!(!game.can_move(Position { x: 0, y: 0 }));
        assert!(game.can_move(Position { x: 1, y: 2 }));
    }

    #[test]
    fn test_path_finding() {
        let game = MazeGame::new(20, 15);
        let path = game.find_shortest_path();
        assert!(path.is_some(), "应该能找到路径");
    }
}