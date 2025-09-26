const std = @import("std");

fn accumulate(nums: []u64) u64 {
    
    if (nums.len == 0) unreachable;
    if ((nums.len % 32) != 0) unreachable;
    var sum: u64  =0;

    for (nums) |n| sum += n; 
    return sum;


}

export fn accumulate_external(nums: [*]u64, len:usize) u64 {
    return accumulate(nums[0..len]);
}
