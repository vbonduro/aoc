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
};

fn parse_start_end(range: []u8) ![2]u64 {
    var range_parts = std.mem.splitScalar(u8, range, '-');
    const start = try std.fmt.parseInt(u64, range_parts.next().?, 10);
    const end = try std.fmt.parseInt(u64, range_parts.next().?, 10);
    return .{ start, end };
}

fn sum_invalid_ids(range: []u8) !u64 {
    const start_end: [2]u64 = try parse_start_end(range);

    var invalid_ids: u64 = 0;
    for (start_end[0]..(start_end[1] + 1)) |id| {
        // Make this a function. is_invalid_id.
        // Function will split decimal, then continue splitting halves to find repeats.
        const id_as_decimal: Decimal = .init(@intCast(id));
        const id_parts: [2]u64 = id_as_decimal.split() catch continue;
        invalid_ids += if (id_parts[0] == id_parts[1]) @intCast(id) else 0;
    }
    return invalid_ids;
}

pub fn main() !void {
    const input = try std.fs.cwd().openFile("input.txt", .{});
    defer input.close();

    var buf: [4096]u8 = undefined;
    var threaded: std.Io.Threaded = .init_single_threaded;
    var reader = input.reader(threaded.io(), &buf);

    var invalid_ids: u64 = 0;
    while (try reader.interface.takeDelimiter(',')) |range| {
        invalid_ids += try sum_invalid_ids(range);
    }

    std.debug.print("Invalid IDs: {d}\n", .{invalid_ids});
}
