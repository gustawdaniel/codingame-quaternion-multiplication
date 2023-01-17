export class Quaternion {
    r: number = 0;
    i: number = 0;
    j: number = 0;
    k: number = 0;

    static parse(input: string): Quaternion[] {
        const qs = (input.match(/\(.*?\)/g) ?? []).map(
            (e: string) => (e
                    .replace('(', '')
                    .replace(')', '')
                    .match(/[-+]?[\d.]*[ijk]?/g) ?? []
            ).filter(v => v).map(
                v => v.replace(/^\+/, '')
            )
        );
        return qs.map((q) => new Quaternion(q));
    }

    static getCoefficient(type: string, input: string): number {
        const coefficient = input.replace(type, '');
        return Number.parseFloat(/\d$/.test(coefficient) ? coefficient : coefficient + '1')
    }

    constructor(args: Array<string>) {
        for (let arg of args) {
            if (arg.includes('i')) {
                this.i = Quaternion.getCoefficient('i', arg);
            } else if (arg.includes('j')) {
                this.j = Quaternion.getCoefficient('j', arg);
            } else if (arg.includes('k')) {
                this.k = Quaternion.getCoefficient('k', arg);
            } else {
                this.r = Number.parseFloat(arg);
            }
        }
    }

}
