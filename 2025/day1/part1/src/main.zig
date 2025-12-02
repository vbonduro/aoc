const std = @import("std");
const part1 = @import("part1");

const Dial = struct {
    marker: u16 = 50,
    pub const dial_size: u16 = 99;

    pub fn left(self: *Dial, clicks: u16) void {
        const difference = if (self.marker > clicks) self.marker - clicks else clicks - self.marker;
        self.marker = difference % dial_size;
    }

    pub fn right(self: *Dial, clicks: u16) void {
        self.marker = (self.marker + clicks) % dial_size;
    }

    pub fn get_marker(self: Dial) u16 {
        return self.marker;
    }
};

const Direction = enum(u8) { Left = 0, Right = 1 };

const RotationError = error{
    InvalidDirection,
};

const Rotation = struct {
    direction: Direction,
    clicks: u16,

    pub fn from_string(str: []const u8) !Rotation {
        var direction = Direction{};
        switch (str[0]) {
            'L' => direction = Direction.Left,
            'R' => direction = Direction.Right,
            else => return RotationError.InvalidDirection,
        }
        const clicks = try std.fmt.parseInt(u16, str[1..], 10);
        return Rotation{ .direction = direction, .clicks = clicks };
    }
};

pub fn main() !void {
    var combo_lock = Dial{};

    const input = try std.fs.cwd().openFile("input.txt", .{});
    defer input.close();

    var buf: [4096]u8 = undefined;
    var reader = input.reader(input, &buf);

    var password: u16 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        const rotation = Rotation.from_string(line);
        switch (rotation.direction) {
            Direction.Left => combo_lock.left(rotation.clicks),
            Direction.Right => combo_lock.right(rotation.clicks),
        }

        if (combo_lock.get_marker() == 0) {
            password += 1;
        }
    }

    std.debug.print("Password: {d}\n", .{password});
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "fuzz example" {
    const Context = struct {
        fn testOne(context: @This(), input: []const u8) anyerror!void {
            _ = context;
            // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
            try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
        }
    };
    try std.testing.fuzz(Context{}, Context.testOne, .{});
}
