import init, { run } from './pkg/roost_web'

main()

async function main() {
    await init()
    postMessage(['ready'])
    onmessage = function (event) {
        if (event.data[0] === 'run') {
            try {
                run(event.data[1])
            } catch (error) {
                console.log('crashed:', error)
            }
            postMessage(['finished'])
        }
    }
}

export function print(message: string) {
    postMessage(['print', message])
    sleep(10)
}

function sleep(ms: number) {
    const start = Date.now()
    while (Date.now() - start < ms);
}
