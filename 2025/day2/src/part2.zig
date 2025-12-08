const std = @import("std");

const DecimalError = error{
    DigitOverflow,
    BadSize,
};

const Decimal = struct {
    value: u64 = 0,
    n_digits: u64 = 1,

    pub fn init(value: u64) Decimal {
        const n_digits = std.math.log10(value) + 1;
        return Decimal{ .value = value, .n_digits = n_digits };
    }

    pub fn length(self: Decimal) u64 {
        return self.n_digits;
    }

    // 12345
    // pos[0] = 5
    // pos[1] = 4 -> 12345 % 100 (10^2) = 45 / 10 (10^1) = 4
    // pos[2] = 3 -> 12345 % 1000 (10^3) = 345 / 100 (10^2) = 3
    pub fn at(self: Decimal, digit: u64) !u64 {
        if (digit >= self.n_digits) {
            return DecimalError.DigitOverflow;
        }
        if (digit == 0) {
            return self.value % 10;
        }
        return (self.value % (std.math.pow(u64, 10, digit + 1))) / (std.math.pow(u64, 10, digit));
    }

    // 1234
    // {12, 34}
    // bottom half 34. size = 4, 1234 % 100 (10^(size/2))
    // top half 12. size = 4, 1234 / 100 (10^(size/2))
    // 12345
    // BadSize
    pub fn split(self: Decimal) ![2]u64 {
        if (self.n_digits % 2 != 0) {
            return error.BadSize;
        }
        const operand: u64 = std.math.pow(u64, 10, self.n_digits / 2);
        return .{ self.value % operand, self.value / operand };
    }

    // Split value into n different parts.
    // 1234 split 2 parts -> 12,34
    // 1234 split 3 parts -> Error
    // 1234 split 4 parts = 1,2,3,4

    // 123456 split 3 parts = 12,34,56
    // i = 0 size=6 n_parts=3 part_size=2 => 12 (123456 / 10000 (10^4) --> 4 is size - part_size
    // i = 1 size=6 n_parts=3 part_size=2 => 34 (123456 / 100 (10^2)) % 100 (10^2); (2==part_size) -> 2 is last 4 - part_size
    // i = 2 size=6 n_parts=3 part_size=2 => 56 (123456 / 1) % 100 (10^2); (2==part_size)

    // 123456789 split 3 parts = 123,456,789
    // i = 0 size=9 n_parts=3 part_size=3 => 123 (123456789 / 1 000 000) (10^6) --> 6 is size - part_size
    // i = 1 size=9 n_parts=3 part_size=3 => 456 (123456789 / 1000 (10^3)) % 1000 (10^3)
    // i = 2 size=9 n_parts=3 part_size=3 => 789 (123456789 / 1) % 1000 (10^3)

    // expo=size - part_size
    // for i=0..n_parts
    // if i == 0
    //     res[i] = (value/10^expo)
    // else
    //     res[i] = (value/10^expo) % (10^part_size)
    // expo=expo - partsize
    pub fn split_n(self: Decimal, allocator: std.mem.Allocator, n_parts: u64) ![]u64 {
        if (self.n_digits % n_parts != 0) {
            return error.BadSize;
        }

        const part_size = self.n_digits / n_parts;
        var buf = try allocator.alloc(u64, n_parts);

        for (0..n_parts) |i| {
            const expo: u64 = self.n_digits - (part_size * (i + 1));
            buf[i] = switch (i) {
                0 => self.value / std.math.pow(u64, 10, expo),
                else => self.value / std.math.pow(u64, 10, expo) % std.math.pow(u64, 10, part_size),
            };
        }

        return buf;
    }
};

fn parse_start_end(range: []u8) ![2]u64 {
    var range_parts = std.mem.splitScalar(u8, range, '-');
    const start = try std.fmt.parseInt(u64, range_parts.next().?, 10);
    const end = try std.fmt.parseInt(u64, range_parts.next().?, 10);
    return .{ start, end };
}

fn all_equal(parts: []u64) bool {
    for (1..parts.len) |i| {
        if (parts[0] != parts[i]) {
            return false;
        }
    }
    return true;
}

fn is_invalid(allocator: std.mem.Allocator, id: u64) !bool {
    var decimal_id: Decimal = .init(id);

    for (2..decimal_id.n_digits + 1) |n_parts| {
        const parts = decimal_id.split_n(allocator, n_parts) catch continue;
        defer allocator.free(parts);
        if (all_equal(parts)) {
            return true;
        }
    }
    return false;
}

fn sum_invalid_ids(allocator: std.mem.Allocator, range: []u8) !u64 {
    const start_end: [2]u64 = try parse_start_end(range);

    var invalid_ids: u64 = 0;
    for (start_end[0]..(start_end[1] + 1)) |id_it| {
        const id: u64 = @intCast(id_it);
        invalid_ids += if (try is_invalid(allocator, id)) id else 0;
    }
    return invalid_ids;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);

    const allocator = gpa.allocator();

    const input = try std.fs.cwd().openFile("input.txt", .{});
    defer input.close();

    var buf: [4096]u8 = undefined;
    var threaded: std.Io.Threaded = .init_single_threaded;
    var reader = input.reader(threaded.io(), &buf);

    var invalid_ids: u64 = 0;
    while (try reader.interface.takeDelimiter(',')) |range| {
        std.debug.print("Processing: {s}\n", .{range});
        invalid_ids += try sum_invalid_ids(allocator, range);
    }

    std.debug.print("Invalid IDs: {d}\n", .{invalid_ids});
}
