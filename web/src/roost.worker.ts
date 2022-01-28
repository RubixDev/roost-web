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
                postMessage(['print', `<span class="bold red">${error}</span><br>`])
            }
            postMessage(['finished'])
        }
    }
}

export function print(message: string) {
    postMessage(['print', message])
}
