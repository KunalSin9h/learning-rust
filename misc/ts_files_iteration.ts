import fs from "fs"

fs.readFileSync("text_file.txt").
    toString().
    split("\n").
    filter((_, idx) => idx % 2 === 0).
    filter((_, idx) => idx > 1 && idx < 4).
    forEach(line => console.log(line));

