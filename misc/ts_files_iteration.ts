import fs from "fs"

fs.readFileSync("text_file.txt").
    toString().
    split("\n").
    filter((_, idx) => idx % 2 === 0).
    forEach(line => console.log(line));

