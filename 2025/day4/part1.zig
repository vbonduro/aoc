const std = @import("std");

fn is_paper(symbol: u8) bool {
    return symbol == '@';
}

const Position = struct {
    row: u32,
    col: u32,
};

fn count_papers_horrizontally(from: Position, n_spaces: usize, grid: [3][]const u8) u32 {
    const n_cols: usize = grid[0].len;
    var n_papers: u32 = 0;
    for (from.col..from.col + n_spaces) |col| {
        if (col >= n_cols) break;
        if (is_paper(grid[from.row][col])) n_papers += 1;
    }
    return n_papers;
}

// Assumes 0,0 is top left corner.

fn count_papers_above(pos: Position, grid: [3][]const u8) u32 {
    if (pos.row == 0) return 0;
    const n_cols: usize = if (pos.col == 0) 2 else 3;
    const col: u32 = if (pos.col == 0) pos.col else pos.col - 1;
    return count_papers_horrizontally(Position{ .row = pos.row - 1, .col = col }, n_cols, grid);
}

fn count_papers_at(pos: Position, grid: [3][]const u8) u32 {
    var n_adjacent_papers: u32 = 0;
    if (pos.col > 0) {
        if (is_paper(grid[pos.row][pos.col - 1])) n_adjacent_papers += 1;
    }
    const n_cols = grid[0].len;
    if (pos.col < n_cols - 1) {
        if (is_paper(grid[pos.row][pos.col + 1])) n_adjacent_papers += 1;
    }
    return n_adjacent_papers;
}

fn count_papers_below(pos: Position, grid: [3][]const u8) u32 {
    if (pos.row == grid.len - 1) return 0;
    const n_cols: usize = if (pos.col == 0) 2 else 3;
    const col: u32 = if (pos.col == 0) pos.col else pos.col - 1;
    return count_papers_horrizontally(Position{ .row = pos.row + 1, .col = col }, n_cols, grid);
}

fn count_adjacent_papers(pos: Position, grid: [3][]const u8) u32 {
    var n_adjacent_papers: u32 = 0;
    n_adjacent_papers += count_papers_above(pos, grid);
    n_adjacent_papers += count_papers_at(pos, grid);
    n_adjacent_papers += count_papers_below(pos, grid);
    return n_adjacent_papers;
}

fn is_an_accessible_paper(pos: Position, grid: [3][]const u8) bool {
    if (!is_paper(grid[pos.row][pos.col])) return false;

    return count_adjacent_papers(pos, grid) < 4;
}

fn count_accessible_papers_for_row(row: u32, grid: [3][]const u8) u32 {
    const n_cols = grid[0].len;

    var n_accessible_papers: u32 = 0;
    for (0..n_cols) |col| {
        if (is_an_accessible_paper(Position{ .row = row, .col = @intCast(col) }, grid)) {
            std.debug.print("x", .{});
            n_accessible_papers += 1;
        } else {
            if (is_paper(grid[row][col])) {
                std.debug.print("@", .{});
            } else std.debug.print(".", .{});
        }
    }
    std.debug.print("\n", .{});
    return n_accessible_papers;
}

pub fn main() !void {
    const input = try std.fs.cwd().openFile("input.txt", .{});
    defer input.close();

    var buf: [4096]u8 = undefined;
    var threaded: std.Io.Threaded = .init_single_threaded;
    var reader = input.reader(threaded.io(), &buf);

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);

    const allocator = gpa.allocator();

    var grid: [3][]u8 = undefined;
    var total_accessible_papers: u64 = 0;
    var fill_row: u8 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        // Maintain a buffer of 3 rows
        // Starting:
        // read 3 rows   [....]
        //               [....]
        //               [....]
        // count on first row
        // count on middle row
        // for each subsequent row:
        //      shift rows up, save new row in last row
        //      count in middle row
        // after end of loop, count in last row
        if (fill_row < grid.len) {
            grid[fill_row] = try allocator.alloc(u8, line.len);
            @memcpy(grid[fill_row], line);
            fill_row += 1;
            if (fill_row == grid.len) {
                total_accessible_papers += count_accessible_papers_for_row(0, grid);
                total_accessible_papers += count_accessible_papers_for_row(1, grid);
            }
        } else {
            @memcpy(grid[0], grid[1]);
            @memcpy(grid[1], grid[2]);
            @memcpy(grid[2], line);
            total_accessible_papers += count_accessible_papers_for_row(1, grid);
        }
    }
    total_accessible_papers += count_accessible_papers_for_row(2, grid);

    for (grid) |row| {
        allocator.free(row);
    }

    std.debug.print("Total Accessible Papers: {d}\n", .{total_accessible_papers});
}
