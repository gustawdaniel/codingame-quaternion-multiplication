import {Quaternion} from "./lib";

process.stdin.on('data', (buff) => {
    const line = buff.toString();
    const qs = Quaternion.parse(line);
    process.stdout.write(qs.reduce((p, n) => p.multiply(n)).format());
})
