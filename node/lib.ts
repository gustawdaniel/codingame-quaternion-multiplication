type Base = 'r' | 'i' | 'j' | 'k';

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

    static multiplyBase(a: Base, b: Base): { c: -1 | 1, d: Base } {
        if (a === 'r') return {c: 1, d: b};
        if (b === 'r') return {c: 1, d: a};
        if (a === b) return {c: -1, d: 'r'};
        const diff = a.charCodeAt(0) - b.charCodeAt(0);
        return {
            c: (diff > 0 ? -1 : 1) * ((diff + 2) % 2 === 0 ? -1 : 1) as -1 | 1,
            d: ['i', 'j', 'k'].find((e) => e !== a && e !== b) as Base
        }
    }

    //      1	i	j	k
    // 1	1	i	j	k
    // i	i	-1	k	-j
    // j	j	-k	-1	i
    // k	k	j	-i	-1
    multiply(a: Quaternion): Quaternion {
        const res = new Quaternion([]);
        for (let p of ['r', 'i', 'j', 'k'] as Array<Base>) {
            for (let n of ['r', 'i', 'j', 'k'] as Array<Base>) {
                const {c, d} = Quaternion.multiplyBase(p, n);
                res[d] += c * this[p] * a[n];
            }
        }
        return res;
    }

    static formatCoefficient(type: Base | '', value: number) {
        const out = `${Math.abs(value) === 1 ? (Math.sign(value) === 1 ? '' : '-') : value}${type}`;
        return /[\dijk]$/.test(out) ? out : `${out}1`;
    }

    format(): string {
        let out = [];
        if (this.i) {
            out.push(Quaternion.formatCoefficient('i', this.i));
        }
        if (this.j) {
            out.push(Quaternion.formatCoefficient('j', this.j));
        }
        if (this.k) {
            out.push(Quaternion.formatCoefficient('k', this.k));
        }
        if (this.r) {
            out.push(Quaternion.formatCoefficient('', this.r));
        }

        if (!out.length) return '0';

        return out.reduce((p, n) => p + (
            p.length && Quaternion.getCoefficient('',n.replace(/[kij]/, '')) > 0 ? `+${n}` : `${n}`), ''
        );
    }
}
