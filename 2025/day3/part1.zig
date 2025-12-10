const std = @import("std");

const Battery = struct { index: usize, joltage: u8 };

pub fn max_battery_in_range(bank: []const u8, start: usize, end: usize) Battery {
    var max_battery = Battery{ .index = 0, .joltage = 0 };
    for (start..end) |i| {
        const joltage: u8 = bank[i] - '0';
        if (joltage == 9) {
            return Battery{ .index = i, .joltage = 9 };
        }
        if (joltage > max_battery.joltage) {
            max_battery = Battery{ .index = i, .joltage = joltage };
        }
    }
    return max_battery;
}

pub fn max_joltage(bank: []const u8) u8 {
    const first_max_battery = max_battery_in_range(bank, 0, bank.len - 1);
    const second_max_battery = max_battery_in_range(bank, first_max_battery.index + 1, bank.len);
    std.debug.print("bank: {s} first {d} second {d}\n", .{ bank, first_max_battery.joltage, second_max_battery.joltage });
    return first_max_battery.joltage * 10 + second_max_battery.joltage;
}

pub fn main() !void {
    const input = try std.fs.cwd().openFile("input.txt", .{});
    defer input.close();

    var buf: [4096]u8 = undefined;
    var threaded: std.Io.Threaded = .init_single_threaded;
    var reader = input.reader(threaded.io(), &buf);

    var total_joltage: u64 = 0;

    while (try reader.interface.takeDelimiter('\n')) |bank| {
        total_joltage += max_joltage(bank);
    }

    std.debug.print("Total Joltage: {d}\n", .{total_joltage});
}
