export function regex_vanilla_string(regex: RegExp) {
    return function(string: string): boolean {
        return regex.test(string)
    }
}

export const memo = (() => {
    const cache = {} as any
    return <V>(key: string, cb: () => V): V  => {
        if (key in cache) return cache[key]
        return cache[key] = cb()
    }
})()

export function sleep(delay = 3000) {
    return new Promise((resolve) => {
        setTimeout(resolve, delay)
    })
}