const std = @import("std");
const builtin  = @import("builtin");

fn accumulate(nums: []u64) u64 {
    // 8 by values can I fit in vector
    //@compileLog(std.simd.suggestVectorLengthForCpu(u64, builtin.cpu));

    if (nums.len == 0) unreachable;
    if ((nums.len % 32) != 0) unreachable; // over allocating enough
    const VLEN = std.simd.suggestVectorLengthForCpu(u64, builtin.cpu) orelse 1 ;
    var sum: [4]@Vector(VLEN, u64)  = .{@splat(0) , @splat(0), @splat(0),@splat(0)};
    
    var cur = nums[0..]; // creates slice
    while(true){
        inline for (&sum) |*x| {
            const chunk : @Vector(VLEN, u64) = cur[0..VLEN].*;
            x.* = x.* +| chunk;
            cur = cur[VLEN..];
        }
        if (cur.len == 0) break;
    }
    var final : @Vector(VLEN, u64) = @splat(0);
    inline for(sum) |s| { final +|= s; }

    var fsum : u64 =0;
    inline for(0..VLEN) |i| { fsum +|= final[i]; }

    return fsum;
}


// this for godblot
export fn accumulate_external(nums: [*]u64, len:usize) u64 {
    return accumulate(nums[0..len]);
}
