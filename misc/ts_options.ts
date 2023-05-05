function returnNumber(x: number | undefined) {
    return (x ?? 0) * 5;
    /*
     * || -> this consider 0 and "" as a falsy value along with null, undefined 
     *
     * ?? -> this consider will NOT consider 0 and "" as falsy values
     */
}

console.log(returnNumber(0));

function practice(nums: number[], idx: number) {
    return (nums[idx] ?? idx) * 5;
}

console.log(practice([1, 2, 3], 0)); // 5 
console.log(practice([1, 2, 3], 1)); // 10
console.log(practice([1, 2, 3], 2)); // 15
console.log(practice([1, 2, 3], 3)); // 15
