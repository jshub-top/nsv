import { createWriteStream } from "fs-extra";
import { get } from "http";


export const combineArrays = function< T >(arrays : T[][]) : T[][] {
    if (arrays.length === 0) {
        return [[]];
    }

    let result = [];
    let first = arrays[0];
    let remaining = arrays.slice(1);

    for (let value of first) {
        let subCombinations = combineArrays(remaining);
        for (let subCombination of subCombinations) {
            // @ts-ignore
            result.push([value].concat(subCombination));
        }
    }

    return result;
}


export const download_file = async function (url: string, save_dir: string) {
    return new Promise<void>((resolve, reject) => {
        const write_file_steam = createWriteStream(save_dir)
        get(url, ( res ) => {
            res.pipe(write_file_steam)
            res.on("close", () => {
                write_file_steam.close()
                resolve()
            })
        })
    })
}
