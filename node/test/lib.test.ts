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
})
