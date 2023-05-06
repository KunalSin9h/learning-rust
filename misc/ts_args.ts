import fs from "fs";

const fileName = process.argv[2];

try {

    let fileContent = fs.readFileSync(fileName).toString();
    fileContent.
        split("\n").
        forEach(line => {
            // if (Number.isInteger(+line)) console.log(line);
            // else console.log("Line not a number");
            const print = parseInt(line); 
            if (isNaN(print)) {
                console.log("Line not a number");
            } else {
                console.log(print);
            }
        });

} catch (error) {
    console.log("File", fileName, "not found");
}


