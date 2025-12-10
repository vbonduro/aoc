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

pub fn max_joltage(bank: []const u8) u64 {
    var max_batteries: [12]Battery = std.mem.zeroes([12]Battery);

    for (0..max_batteries.len) |i| {
        const start: usize = if (i == 0) 0 else max_batteries[i - 1].index + 1;
        const max_battery: Battery = max_battery_in_range(bank, start, bank.len - (max_batteries.len - (i + 1)));
        max_batteries[i] = max_battery;
    }

    var joltage: u64 = 0;
    for (max_batteries, 0..) |battery, i| {
        joltage += (battery.joltage * std.math.pow(u64, 10, max_batteries.len - (i + 1)));
    }
    return joltage;
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
