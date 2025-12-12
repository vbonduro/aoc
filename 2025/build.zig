const std = @import("std");

pub fn add_aoc_artifact(b: *std.Build, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode, comptime day: []const u8, comptime part: []const u8) void {
    const exe = b.addExecutable(.{
        .name = day ++ part,
        .root_module = b.createModule(.{
            .root_source_file = b.path(day ++ "/" ++ part ++ ".zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    b.installArtifact(exe);

    const run_step = b.step("run_" ++ day ++ part, "Run " ++ day ++ " " ++ part);
    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
}

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    add_aoc_artifact(b, target, optimize, "day3", "part1");
    add_aoc_artifact(b, target, optimize, "day3", "part2");
    add_aoc_artifact(b, target, optimize, "day4", "part1");
}
