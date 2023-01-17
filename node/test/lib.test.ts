import {Quaternion} from "../lib";

describe('quaternion', () => {
    it('simple parse', () => {
        const qs = Quaternion.parse('(i+j)');
        expect(qs.length).toEqual(1);
        expect(qs[0].r).toEqual(0);
        expect(qs[0].i).toEqual(1);
        expect(qs[0].j).toEqual(1);
        expect(qs[0].k).toEqual(0);
    });

    it('complex parse', () => {
        const qs = Quaternion.parse('(9+i-j)(k-8.4j)');
        expect(qs.length).toEqual(2);
        expect(qs[0].r).toEqual(9);
        expect(qs[0].i).toEqual(1);
        expect(qs[0].j).toEqual(-1);
        expect(qs[0].k).toEqual(0);

        expect(qs[1].r).toEqual(0);
        expect(qs[1].i).toEqual(0);
        expect(qs[1].j).toEqual(-8.4);
        expect(qs[1].k).toEqual(1);
    });

    it('multiply base', () => {
        expect(Quaternion.multiplyBase('r', 'r')).toEqual({c: 1, d: 'r'});
        expect(Quaternion.multiplyBase('r', 'i')).toEqual({c: 1, d: 'i'});
        expect(Quaternion.multiplyBase('r', 'j')).toEqual({c: 1, d: 'j'});
        expect(Quaternion.multiplyBase('r', 'k')).toEqual({c: 1, d: 'k'});

        expect(Quaternion.multiplyBase('i', 'r')).toEqual({c: 1, d: 'i'});
        expect(Quaternion.multiplyBase('i', 'i')).toEqual({c: -1, d: 'r'});
        expect(Quaternion.multiplyBase('i', 'j')).toEqual({c: 1, d: 'k'});
        expect(Quaternion.multiplyBase('i', 'k')).toEqual({c: -1, d: 'j'});

        expect(Quaternion.multiplyBase('j', 'r')).toEqual({c: 1, d: 'j'});
        expect(Quaternion.multiplyBase('j', 'i')).toEqual({c: -1, d: 'k'});
        expect(Quaternion.multiplyBase('j', 'j')).toEqual({c: -1, d: 'r'});
        expect(Quaternion.multiplyBase('j', 'k')).toEqual({c: 1, d: 'i'});

        expect(Quaternion.multiplyBase('k', 'r')).toEqual({c: 1, d: 'k'});
        expect(Quaternion.multiplyBase('k', 'i')).toEqual({c: 1, d: 'j'});
        expect(Quaternion.multiplyBase('k', 'j')).toEqual({c: -1, d: 'i'});
        expect(Quaternion.multiplyBase('k', 'k')).toEqual({c: -1, d: 'r'});
    })

    it('simple multiply', () => {
        const res = (new Quaternion(['1']))
            .multiply(new Quaternion(['1']))

        expect(res.r).toEqual(1);
        expect(res.i).toEqual(0);
        expect(res.j).toEqual(0);
        expect(res.k).toEqual(0);
    })

    it('complex multiply', () => {
        const res = (new Quaternion(['2i', '2j']))
            .multiply(new Quaternion(['j', '1']))

        expect(res.r).toEqual(-2);
        expect(res.i).toEqual(2);
        expect(res.j).toEqual(2);
        expect(res.k).toEqual(2);
    })

    it('format', () => {
        expect((new Quaternion([]).format())).toEqual('0');
        expect((new Quaternion(['1']).format())).toEqual('1');
        expect((new Quaternion(['i', '1']).format())).toEqual('i+1');
        expect((new Quaternion(['i', '-3.4j', '1']).format())).toEqual('i-3.4j+1');
        expect((new Quaternion(['j', 'k']).format())).toEqual('j+k');

    })

    it('e2e medium', () => {
        const cases = [
            {
                in: '(i+j)(k)',
                out: 'i-j'
            },
            {
                in: '(i+j+20)(j-9)',
                out: '-9i+11j+k-181'
            },
            {
                in: '(10i)(10j-k+1)(-99i+j-10k+7)(4)',
                out: '-520i-38920j+6800k+7920'
            },
            {
                in: '(i+j+k+1)(i+2j+4k+8)(i+3j+9k+27)(i+j+8k+8)(i-j+k-10)(99i-j+k-1)(k)(j)(i)(3)',
                out: '11415288i-8751432j-5206896k+9766704'
            }
        ]
        for (const c of cases) {
            const qs = Quaternion.parse(c.in);
            const out = qs.reduce((p, n) => p.multiply(n)).format();
            expect(out).toEqual(c.out);
        }
    })
})
