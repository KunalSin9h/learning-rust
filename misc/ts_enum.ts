enum Color {
    Red,
    Green,
    Blue,
}

function printColor(color: Color) {
    switch (color) {
        case Color.Red: 
            console.log("red");
            break;
        case Color.Green:
            console.log("green");
            break;
        case Color.Blue:
            console.log("blue");
            break;
    }
}

printColor(Color.Red);
printColor(Color.Green);
printColor(Color.Blue);

type Custom = {
    age: number,
    name: string,
}

type Item = number | string | Custom;

function append(items: Item[]) {
    items.push("Hello, FEM");
}

const items: Item[] = [1, 23, "Hello"]; 
append(items);
append([1, 2, 3]);
console.log(items);
