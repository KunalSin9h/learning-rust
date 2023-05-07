interface Area {
    area(): number;
}

class Rectangle implements Area {
    constructor(
        public x: number,
        public y: number,
        public height: number,
        public width: number,
    ) {}

    area(): number {
        return this.width * this.height;
    }

}

class Circle implements Area {
    constructor(
        public x: number,
        public y: number,
        public radius: number,
    ) {}

    area(): number {
        return Math.PI * this.radius * this.radius;
    }
}

function printArea(something: Area) {
    console.log(something.area());
}


const rec = new Rectangle(1, 1, 1, 1);
const c = new Circle(1, 1, 1);
printArea(rec);
printArea(c);
