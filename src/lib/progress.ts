import cliProgress from "cli-progress";
export interface ProgressParams {
    total: number
    current: number
    type: "start"|"update"|"end"
}
export function progress(title: string) {
    const bar = new cliProgress.SingleBar({
        format: `${title} [{bar}] {percentage}% | ETA: {eta}s | {value}/{total} {duration_formatted}`
    }, cliProgress.Presets.rect)
    return function ({ total, current, type }: ProgressParams) {
        if (type === "start") bar.start(total, current)
        if (type === "update") bar.update(current)
        if (type === "end") bar.stop()
    }
}
